use poppler::PopplerDocument;
use tesseract::Tesseract;
use cairo::{Context, Format, ImageSurface};
use image::{ImageBuffer, Luma};
use std::error::Error;
use std::path::Path;

fn main() {
    println!("PDF OCR Service - Enhanced Image Processing");

    // Check if a test PDF file is provided as argument
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <pdf_file> [--basic]", args[0]);
        println!("Example: {} test.pdf", args[0]);
        println!("         {} test.pdf --basic  (use basic OCR without image enhancement)", args[0]);
        return;
    }

    let pdf_file = &args[1];
    let use_basic = args.len() > 2 && args[2] == "--basic";

    // Choose OCR method
    let result = if use_basic {
        println!("Using basic OCR mode...");
        extract_text_with_basic_ocr(pdf_file)
    } else {
        println!("Using enhanced image OCR mode...");
        extract_text_with_ocr(pdf_file)
    };

    match result {
        Ok(text) => {
            println!("Successfully extracted text from '{}':", pdf_file);
            println!("--- BEGIN TEXT ---");
            println!("{}", text.trim());
            println!("--- END TEXT ---");
        },
        Err(e) => eprintln!("Error processing '{}': {}", pdf_file, e),
    }
}

/// Preprocess image for better OCR results
///
/// This function applies various image enhancement techniques:
/// 1. Converts to grayscale
/// 2. Enhances contrast
/// 3. Applies noise reduction
/// 4. Optimizes for text recognition
fn preprocess_image_for_ocr(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    // Load the image
    let img = image::open(input_path)?;

    // Convert to grayscale for better OCR performance
    let gray_img = img.to_luma8();

    // Apply contrast enhancement
    let enhanced_img = enhance_contrast(&gray_img);

    // Save the preprocessed image
    enhanced_img.save(output_path)?;

    Ok(())
}

/// Enhance contrast of grayscale image
fn enhance_contrast(img: &ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut enhanced = img.clone();

    // Simple contrast enhancement using histogram stretching
    let mut min_val = 255u8;
    let mut max_val = 0u8;

    // Find min and max pixel values
    for pixel in img.pixels() {
        let val = pixel[0];
        min_val = min_val.min(val);
        max_val = max_val.max(val);
    }

    // Avoid division by zero
    if max_val == min_val {
        return enhanced;
    }

    // Stretch histogram to full range
    let range = max_val - min_val;
    for pixel in enhanced.pixels_mut() {
        let old_val = pixel[0];
        let new_val = ((old_val - min_val) as f32 / range as f32 * 255.0) as u8;
        pixel[0] = new_val;
    }

    enhanced
}

/// Configure Tesseract for optimal image OCR
fn configure_tesseract_for_images(image_path: &str) -> Result<Tesseract, Box<dyn Error>> {
    let tesseract = Tesseract::new(None, Some("eng"))?
        .set_variable("tessedit_pageseg_mode", "1")? // Automatic page segmentation with OSD
        .set_variable("tessedit_ocr_engine_mode", "1")? // Neural nets LSTM engine only
        .set_variable("tessedit_char_whitelist",
            "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz.,!?:;()[]{}\"'-+*/=@#$%^&<>|\\~`")?
        .set_variable("textord_really_old_xheight", "1")?
        .set_variable("textord_min_xheight", "10")?
        .set_image(image_path)?;

    Ok(tesseract)
}

/// Extract text from PDF using basic OCR (for comparison)
///
/// This function uses basic Tesseract settings without image preprocessing
fn extract_text_with_basic_ocr(pdf_path: &str) -> Result<String, Box<dyn Error>> {
    // Check if file exists
    if !Path::new(pdf_path).exists() {
        return Err(format!("PDF file not found: {}", pdf_path).into());
    }

    // Open PDF document
    let doc = PopplerDocument::new_from_file(pdf_path, "")?;
    let mut all_text = String::new();

    // Process each page
    let num_pages = doc.get_n_pages();
    for page_num in 0..num_pages {
        let page = doc.get_page(page_num).ok_or("Failed to get page")?;

        // Get page dimensions
        let (width, height) = page.get_size();
        println!("Processing page {} ({}x{})", page_num + 1, width, height);

        // Create Cairo image surface for rendering
        let surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32)?;
        let ctx = Context::new(&surface)?;

        // Set white background
        ctx.set_source_rgb(1.0, 1.0, 1.0);
        ctx.paint()?;

        // Render PDF page to Cairo surface
        page.render(&ctx);

        // Save the rendered page as a temporary PNG file
        let temp_filename = format!("temp_page_{}.png", page_num);
        let mut file = std::fs::File::create(&temp_filename)?;
        surface.write_to_png(&mut file)?;

        // Initialize basic Tesseract OCR engine
        let mut tesseract = Tesseract::new(None, Some("eng"))?
            .set_image(&temp_filename)?;

        // Extract text using basic OCR
        let page_text = tesseract.get_text()?;

        // Clean up temporary file
        std::fs::remove_file(&temp_filename)?;
        all_text.push_str(&page_text);
        all_text.push('\n');
    }

    Ok(all_text)
}

/// Extract text from PDF using enhanced OCR for images
///
/// This function:
/// 1. Opens the PDF using poppler
/// 2. Renders each page as an image
/// 3. Applies image preprocessing for better OCR
/// 4. Uses optimized Tesseract settings for images
/// 5. Returns the combined text from all pages
fn extract_text_with_ocr(pdf_path: &str) -> Result<String, Box<dyn Error>> {
    // Check if file exists
    if !Path::new(pdf_path).exists() {
        return Err(format!("PDF file not found: {}", pdf_path).into());
    }

    // Open PDF document
    let doc = PopplerDocument::new_from_file(pdf_path, "")?;
    let mut all_text = String::new();

    // Process each page
    let num_pages = doc.get_n_pages();
    for page_num in 0..num_pages {
        let page = doc.get_page(page_num).ok_or("Failed to get page")?;

        // Get page dimensions
        let (width, height) = page.get_size();
        println!("Processing page {} ({}x{})", page_num + 1, width, height);

        // Create Cairo image surface for rendering
        let surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32)?;
        let ctx = Context::new(&surface)?;

        // Set white background
        ctx.set_source_rgb(1.0, 1.0, 1.0);
        ctx.paint()?;

        // Render PDF page to Cairo surface
        page.render(&ctx);

        // Save the rendered page as a temporary PNG file
        let temp_filename = format!("temp_page_{}.png", page_num);
        let mut file = std::fs::File::create(&temp_filename)?;
        surface.write_to_png(&mut file)?;

        // Preprocess the image for better OCR
        let processed_filename = format!("processed_page_{}.png", page_num);
        preprocess_image_for_ocr(&temp_filename, &processed_filename)?;

        // Initialize Tesseract OCR engine with enhanced settings for images
        let mut tesseract = configure_tesseract_for_images(&processed_filename)?;

        // Extract text using OCR
        let page_text = tesseract.get_text()?;

        // Clean up temporary files
        std::fs::remove_file(&temp_filename)?;
        std::fs::remove_file(&processed_filename)?;
        all_text.push_str(&page_text);
        all_text.push('\n');
    }

    Ok(all_text)
}
