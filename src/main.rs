use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

fn convert_html_to_pdf(html_file: &str, pdf_file: &str) -> io::Result<()> {
    if !Path::new(html_file).exists() {
        eprintln!("❌ HTML file not found: {}", html_file);
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }

    let output = Command::new("wkhtmltopdf")
        .arg(html_file)
        .arg(pdf_file)
        .output()?;

    if output.status.success() {
        println!("✅ Successfully created: {}", pdf_file);
    } else {
        eprintln!("❌ Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

fn merge_pdfs(pdf_files: &[String], output_file: &str) -> io::Result<()> {
    if pdf_files.is_empty() {
        return Err(io::Error::new(io::ErrorKind::Other, "No PDF files to merge"));
    }

    let mut command = Command::new("pdftk");
    for pdf_file in pdf_files {
        command.arg(pdf_file);
    }
    
    command.arg("cat").arg("output").arg(output_file);

    let output = command.output()?;
    
    if output.status.success() {
        println!("✅ Successfully merged PDFs into: {}", output_file);
        Ok(())
    } else {
        eprintln!("❌ Error merging PDFs: {}", String::from_utf8_lossy(&output.stderr));
        Err(io::Error::new(io::ErrorKind::Other, "Failed to merge PDFs"))
    }
}

fn delete_files_with_extension(dir: &str, extension: &str, exclude_file: Option<&str>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == extension) {
                if let Some(exclude) = exclude_file {
                    if path.file_name().map_or(false, |f| f == exclude) {
                        continue;
                    }
                }
                if let Err(e) = fs::remove_file(&path) {
                    eprintln!("❌ Failed to delete {}: {}", path.display(), e);
                } else {
                    println!("🗑️ Deleted: {}", path.display());
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("❌ Usage: {} <html_directory>", args[0]);
        std::process::exit(1);
    }

    let input_dir = &args[1];
    let final_pdf = format!("{}/merged_output.pdf", input_dir);
    let mut pdf_files = Vec::new();

    let html_files = fs::read_dir(input_dir).expect("Failed to read input directory");
    for file in html_files {
        if let Ok(entry) = file {
            let html_path = entry.path();
            if html_path.extension().map_or(false, |ext| ext == "html") {
                let pdf_path = format!("{}/{}.pdf", 
                    input_dir, 
                    html_path.file_stem().unwrap().to_string_lossy()
                );
                if let Err(e) = convert_html_to_pdf(html_path.to_str().unwrap(), &pdf_path) {
                    eprintln!("Error converting {}: {}", html_path.display(), e);
                } else {
                    pdf_files.push(pdf_path);
                }
            }
        }
    }

    pdf_files.sort();

    if !pdf_files.is_empty() {
        if let Err(e) = merge_pdfs(&pdf_files, &final_pdf) {
            eprintln!("Error merging PDFs: {}", e);
        } else {
            delete_files_with_extension(input_dir, "html", None);
            delete_files_with_extension(input_dir, "pdf", Some("merged_output.pdf"));
            println!("✅ Processing completed successfully!");
        }
    } else {
        println!("No PDF files were generated to merge.");
    }
}
