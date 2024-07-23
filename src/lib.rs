use printpdf::*;
use scraper::{Html, Selector};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use anyhow::{Context, Result};

// Convert HTML to plain text
fn html_to_text(html: &str) -> String {
    let document = Html::parse_document(html);
    let body_selector = Selector::parse("body").unwrap();
    let mut text_content = String::new();

    // Select and process body content
    if let Some(body) = document.select(&body_selector).next() {
        for node in body.text() {
            text_content.push_str(node.trim());
            text_content.push('\n'); // Add a newline for each text block
        }
    }

    text_content
}

// Convert plain text to PDF
pub fn html_to_pdf(html: &str, output_path: &Path) -> Result<()> {
    let text = html_to_text(html);
    let (document, page1, layer1) = PdfDocument::new("HTML to PDF", Mm(210.0), Mm(297.0), "Layer 1");

    // Load a built-in font and create an indirect font reference
    let font = document.add_builtin_font(BuiltinFont::Helvetica)
        .context("Failed to add built-in font")?;
    let font_size = 12.0;
    let mut y_position = 287.0; // Start near the top of the page

    // Prepare file output
    let file = File::create(output_path).context("Failed to create output file")?;
    let mut buffer = BufWriter::new(file);

    // Get the current page's layer
    let mut current_layer = document.get_page(page1).get_layer(layer1);

    // Render text onto the PDF
    for line in text.lines() {
        if !line.trim().is_empty() {
            // Ensure text fits within the page
            if y_position < 10.0 {
                // Create a new page if necessary
                let (new_page, new_layer) = document.add_page(Mm(210.0), Mm(297.0), "New Page");
                current_layer = document.get_page(new_page).get_layer(new_layer);
                y_position = 287.0; // Reset y-position on new page
            }

            // Add the text to the PDF
            current_layer.use_text(line, font_size, Mm(10.0), Mm(y_position), &font);
            y_position -= font_size + 2.0; // Move to the next line
        }
    }

    // Save the PDF and ensure it's written
    document.save(&mut buffer).context("Failed to save PDF")?;

    Ok(())
}
