use pdf_extract::{extract_text};
use std::path::{PathBuf};


#[derive(Debug)]
pub struct PDFEmailInfo {
    pub from_line: String,
    pub date_line: String,
    pub subject_line: String,
}

pub fn make_header(pdf_path: &String) -> Result<PDFEmailInfo, String> {
    let result = get_lines_from_pdf(&PathBuf::from(pdf_path.to_string()));
    return match result {
        Ok(lines) => {
            let line_ref = lines.iter().map(AsRef::as_ref).collect::<Vec<_>>();
            let from_line = get_from_line(&line_ref);
            let date_line = get_date_line(&line_ref);
            let subject_line = get_subject_line(&line_ref);
            if date_line.is_err() {
                Err(date_line.err().unwrap())
            } else if from_line.is_err() {
                Err(from_line.err().unwrap())
            } else if subject_line.is_err() {
                Err(subject_line.err().unwrap())
            } else {
                Ok(
                    PDFEmailInfo {
                        from_line: from_line.clone().unwrap().to_string(),
                        date_line: date_line.clone().unwrap().to_string(),
                        subject_line: subject_line.clone().unwrap().to_string()
                    }
                )
            }
        }
        Err(e) => {
            return Err(e)
        }
    }
}

fn get_lines_from_pdf(pdf_path: &PathBuf) -> Result<Vec<String>, String> {
    let binding = extract_text(&pdf_path);
    return match binding {
        Ok(bind) => {
            let text: &str = bind.as_str();
            let lines = text.split("\n").into_iter().filter(|&x| !x.is_empty()).map(|x| x.trim()).collect::<Vec<&str>>();
            Ok(lines.into_iter().map(|x| String::from(x)).collect())
        }
        Err(e) => {
            return Err(format!("Could not read in pdf: {name}; {err}", name = pdf_path.to_str().unwrap_or("NO NAME"), err = e.to_string()));
        }
    }
}
fn get_line_that_starts_with<'a>(string: &'a str, lines: &Vec<&'a str>)-> Result<&'a str, String> {
    let index = lines.iter().position(|&r| r.starts_with(string));
    return match index {
        Some(ind) => return Ok(lines.get(ind).unwrap()),
        _ => Err(format!("Could not find {line_name} line", line_name = string))
    }
}

fn get_from_line<'a>(lines: &'a Vec<&'a str>) -> Result<&'a str, String> {
   return get_line_that_starts_with("From", lines)
}

fn get_date_line<'a>(lines: &'a Vec<&'a str>) -> Result<&'a str, String> {
    return get_line_that_starts_with("Sent", lines)
}

fn get_subject_line<'a >(lines: &'a Vec<&'a str>) -> Result<&'a str, String> {
    return get_line_that_starts_with("Subject", lines)
}