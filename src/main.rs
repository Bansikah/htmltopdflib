use std::env;
use std::fs;
use std::path::Path;
use anyhow::Context;
use anyhow::Result;
use html_to_pdf_lib::html_to_pdf; // Import your library function

fn main() -> Result<()> {
    // Ensure the correct number of arguments
    if env::args().len() != 2 {
        eprintln!("Usage: {} <input_html_file>", env::args().next().unwrap());
        std::process::exit(1);
    }

    // Get the input file path from the command line arguments
    let input_path = env::args().nth(1).expect("No input file provided");
    let output_path = Path::new("output.pdf");

    // Read the HTML file content
    let html = fs::read_to_string(&input_path)
        .context("Failed to read input file")?;

    // Convert HTML to PDF
    html_to_pdf(&html, output_path)?;

    println!("PDF generated successfully at {}", output_path.display());

    Ok(())
}
