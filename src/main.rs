use std::path::Path;
use crate::pdf_parser::PDFEmailInfo;
use std::{env, fs};
use std::fs::metadata;

mod pdf_parser;
mod header_parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Need two command line arguments. First argument is the source director. Second argument is the destination directory.")
    }
    let source_pdf_directory = args.get(1).unwrap();
    let dest_pdf_directory = args.get(2).unwrap();

    let source_check = metadata(source_pdf_directory).expect(format!("Source directory {} not found", source_pdf_directory).as_str());
    if !source_check.is_dir() {
        panic!("{}", format!("Source directory {} not found", source_pdf_directory))
    }

    let source_check = metadata(dest_pdf_directory).expect(format!("Destination directory {} not found", dest_pdf_directory).as_str());
    if !source_check.is_dir() {
        panic!("{}", format!("Destination directory {} not found", dest_pdf_directory))
    }


    let pdf_files = get_pdf_files(source_pdf_directory);
    println!("Found {} pdf files", pdf_files.len());

    for pdf in pdf_files {
        process_pdf(pdf, dest_pdf_directory)
    }
}

fn process_pdf(pdf_file: String, destination_dir: &String) {
    match pdf_parser::make_header(&pdf_file) {
        Ok(header) => {
            match get_file_name(&header) {
                Ok(new_file_name) => {
                    match copy_pdf(pdf_file.clone(), new_file_name, destination_dir) {
                        Ok(_) => println!("Processed {}", pdf_file),
                        Err(e) => println!("{}", e.to_string())
                    }
                }
                Err(e) => {
                    println!("Could process pdf: {}", e.to_string());
                }
            }
        }
        Err(e) => {
            println!("Could process pdf: {}", e.to_string());
        }
    }
}

fn copy_pdf(pdf_file: String, new_name: String, destination_dir: &String) -> Result<(), String> {
    let new_dir = Path::new(destination_dir).join(new_name.clone());
    return match fs::create_dir(new_dir.clone()) {
        Ok(_) => {
            let dest_file = new_dir.clone().join(format!("{}{}", new_name, String::from(".pdf")));
            match fs::copy(pdf_file, dest_file) {
                Err(e) => {
                    Err(format!("Could not copy file {} with error: {}", new_dir.display().to_string(), e.to_string()))
                }
                _ => Ok(())
            }
        }
        Err(e) => {
            return Err(format!("Could not create directory: {} with error: {}", new_dir.display().to_string(), e.to_string()));
        }
    }
}

fn get_pdf_files(source_dir: &str) -> Vec<String> {
    match  fs::read_dir(source_dir) {
        Ok(paths) => {
            return paths
                .into_iter()
                .map(|x| x.unwrap().path().to_str().unwrap().clone().to_string())
                .collect::<Vec<String>>()
                .into_iter()
                .filter(|x| x.ends_with(".pdf"))
                .collect::<Vec<String>>();
        }
        Err(e) => {
            panic!("{}", format!("Could not read directory: {} with error: {}", source_dir, e.to_string()));
        }
    }
}

fn get_file_name(header: &PDFEmailInfo) -> Result<String, String> {
    let max_len = 35;
    let email = header_parser::format_email(header.from_line.clone())?;
    let date =  header_parser::format_date(header.date_line.clone())?;
    let subject =  header_parser::format_subject(header.subject_line.clone());

    let mut file_name =  format!("{}_{}_{}", date, email, subject);
    return if file_name.len() < max_len {
        Ok(file_name)
    } else {
        Ok(file_name.drain(0..max_len).collect())
    }
}