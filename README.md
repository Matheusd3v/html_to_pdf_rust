# HTML to PDF Conversion and Merging 📄✨🛠️

This project includes a Rust application for converting HTML files to PDF and merging them, alongside a JavaScript function to execute the Rust binary from a Node.js environment. 🚀📂📑

## Features 🌟🔥🚀
- Converts multiple HTML files to PDF using `wkhtmltopdf`
- Merges generated PDFs into a single file using `pdftk`
- Deletes intermediate files after merging
- Provides a Node.js function to execute the Rust binary

## Requirements 🛠️📌📦
- Rust (for building the Rust binary)
- Node.js (for executing the Rust binary from JavaScript)
- `wkhtmltopdf` (for converting HTML to PDF)
- `pdftk` (for merging PDFs)

## Rust Implementation 🦀📜⚙️

### Functions 🏗️🔢📝
1. **convert_html_to_pdf(html_file: &str, pdf_file: &str)**
   - Converts an HTML file to a PDF using `wkhtmltopdf`

2. **merge_pdfs(pdf_files: &[String], output_file: &str)**
   - Merges multiple PDFs into a single file using `pdftk`

3. **delete_files_with_extension(dir: &str, extension: &str, exclude_file: Option<&str>)**
   - Deletes files of a given extension in a directory (excluding a specific file if provided)

4. **main()**
   - Reads an input directory
   - Converts all `.html` files to PDFs
   - Merges the generated PDFs
   - Cleans up intermediate files

### Usage 🚀🖥️⚡
#### Build and Run 🏗️🔧📜
```sh
cargo build --release
./target/release/html_to_pdf_all <html_directory>
```

## JavaScript Implementation 📜🖥️⚡

### Function: `processHtmlToPdf(htmlDirectoryPath, rustBinaryPath = null)`
Executes the Rust binary with a specified HTML directory path. 🛠️📜🚀

#### Usage 🔧💡✅
```js
const { processHtmlToPdf } = require("./path_to_script");

processHtmlToPdf("./html_files")
  .then(() => console.log("Processing completed!"))
  .catch((error) => console.error("Error:", error));
```

### Handling Output 📥🔍📤
- Captures and logs `stdout` and `stderr` from the Rust process
- Resolves on success, rejects with an error if the process fails

## Installation and Setup 🛠️📦🔄
1. Install dependencies:
   ```sh
   sudo apt install wkhtmltopdf pdftk # Linux/macOS
   ```
2. Build the Rust project:
   ```sh
   cargo build --release
   ```
3. Execute from Node.js using:
   ```js
   processHtmlToPdf("/path/to/html/files");
   ```

## Error Handling ⚠️🛑🔍
- Checks for missing HTML files
- Reports errors from external commands
- Handles Rust process execution failures in Node.js

## Notes 📝💡📌
- Ensure `wkhtmltopdf` and `pdftk` are installed and accessible in your system's `PATH`.
- The Rust binary must be compiled before using it in the Node.js script.
- Update `rustBinaryPath` if the binary is stored in a custom location.

## License 📜✅🌍
MIT License

