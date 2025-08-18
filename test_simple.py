#!/usr/bin/env python3
"""
Simple script to create a test PDF with text for OCR testing.
"""

from reportlab.pdfgen import canvas
from reportlab.lib.pagesizes import letter

def create_test_pdf():
    filename = "test.pdf"
    c = canvas.Canvas(filename, pagesize=letter)
    
    # Add some text to the PDF
    c.setFont("Helvetica", 24)
    c.drawString(100, 750, "Hello, World!")
    
    c.setFont("Helvetica", 16)
    c.drawString(100, 700, "This is a test PDF for OCR.")
    c.drawString(100, 670, "It contains simple text that should be")
    c.drawString(100, 640, "easily recognizable by Tesseract.")
    
    c.setFont("Helvetica", 12)
    c.drawString(100, 600, "Numbers: 1234567890")
    c.drawString(100, 580, "Special chars: !@#$%^&*()")
    
    c.save()
    print(f"Created {filename}")

if __name__ == "__main__":
    create_test_pdf()
