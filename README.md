# Convert HTML to pdf library Rust
This repository contains the htmltopdf Rust library, which converts HTML to PDF. Below are instructions for integrating and using this library in different environments.

## Action Status
![Build-status](https://github.com/Bansikah/htmltopdflib/actions/workflows/build.yml/badge.svg?event=push)

## Prerequisites

- Rust installed on your system
- Java Development Kit (JDK) installed
- Maven installed

## Building the Rust Library

1. **Clone the Repository**

   Clone the repository containing the `html_to_pdf_lib` Rust library.

   ```sh
   git clone https://github.com/Bansikah/htmltopdflib
   cd htmltopdflib
  ```
Build the Rust Library

Build the Rust library to generate the shared library file (.so, .dll, or .dylib) for JNI.

  ```sh
  cargo build --release
  cargo run file.html
  ```
The shared library will be located in the target/release directory. The filename will be html_to_pdf_lib.<extension>.

## Usage Rust project

To use `html_to_pdf_lib` in your Rust project, follow these steps:

1. Add the library as a dependency in your `Cargo.toml`:

    ```toml
    [dependencies]
    html_to_pdf_lib = "0.1.0"
    ```

2. Use the library in your Rust code:

    ```rust
    use html_to_pdf_lib::convert_html_to_pdf;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let html_content = r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>Sample PDF</title>
            </head>
            <body>
                <h1>Hello, World!</h1>
                <p>This is a sample PDF generated from HTML.</p>
            </body>
            </html>
        "#;

        let output_path = "output.pdf";
        convert_html_to_pdf(html_content, output_path)?;

        println!("PDF generated successfully at {}", output_path);

        Ok(())
    }
    ```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
