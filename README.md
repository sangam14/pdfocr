# PDF OCR Service

A Rust-based PDF OCR service that extracts text from PDF files using Tesseract OCR and Poppler for PDF rendering.

## Features

- **PDF to Image Conversion**: Uses Poppler to render PDF pages as images
- **OCR Text Extraction**: Uses Tesseract to extract text from rendered images
- **Multi-page Support**: Processes all pages in a PDF document
- **Error Handling**: Robust error handling for file operations and OCR processing

## Dependencies

### System Requirements

```bash
brew install tesseract poppler pkg-config
```

### Rust Dependencies

The following crates are used:

- `poppler` - PDF rendering
- `tesseract` - OCR text extraction
- `cairo-rs` - Image surface creation
- `image` - Image processing utilities
- `actix-web` - Web framework (for future API development)
- `actix-multipart` - File upload handling

## Building

```bash
# Set library path for linking
export LIBRARY_PATH=/opt/homebrew/Cellar/poppler/25.08.0/lib:$LIBRARY_PATH

# Build the project
cargo build --release
```

## Usage

```bash
# Run OCR on a PDF file
./target/release/pdfocr input.pdf
```

### Example

```bash
# Create a test PDF
echo "This is a test PDF for OCR" > test.txt
echo "It should extract this text" >> test.txt
cupsfilter test.txt > test.pdf

# Run OCR
LIBRARY_PATH=/opt/homebrew/Cellar/poppler/25.08.0/lib:$LIBRARY_PATH ./target/debug/pdfocr test.pdf
```

## How It Works

1. **PDF Loading**: Opens the PDF using Poppler
2. **Page Rendering**: Each page is rendered to a Cairo image surface
3. **Image Processing**: The surface is saved as a temporary PNG file
4. **OCR Processing**: Tesseract processes the image to extract text
5. **Text Aggregation**: Text from all pages is combined and returned

## Future Enhancements

- REST API endpoints for file upload and processing
- Support for multiple languages
- Image preprocessing for better OCR accuracy
- Batch processing capabilities
- Output format options (JSON, plain text, etc.)