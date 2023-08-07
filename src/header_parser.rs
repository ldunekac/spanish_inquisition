use dateparser::parse;

pub fn format_subject(line: String) -> String{
    let underscore: char = '_';
    let dash: char = '-';
    let space: char = ' ';

    return line
        .split_whitespace()
        .collect::<Vec<_>>()
        .drain(1..)
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .filter(|&x| x.is_alphanumeric() || x == dash || x == underscore || x == space)
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace(" ", "_");
}

pub fn format_date(line: String) -> Result<String, String> {
    let mut parts: Vec<_> = line.split(" ").collect::<Vec<_>>();
    let date_vec = parts.drain(2..).take(5).collect::<Vec<_>>();  // remove the "Sent: <day> part of the message
    if date_vec.len() == 5 {
        let date_string = date_vec.join(" ");
        match parse(&date_string) {
            Ok(date) => return Ok(date.format("%Y%M%d").to_string()),
            Err(e) => return Err(format!("Could not parse date from the line : {}; with the error : {}",line, String::from(e.to_string())))
        }
    }
    return Err(String::from(format!("Could not pare date from the line {}", line)));
}


pub fn format_email(line: String) -> Result<String, String> {
    let mut parts: Vec<_> = line.split(" ").collect::<Vec<_>>().into_iter().map(|x| x.replace(",", "")).collect();
    parts.drain(0..1);

    return if !parts.is_empty() {
        if is_email(parts.get(0).unwrap()) {
           return Ok(parse_email(parts.get(0).unwrap()))
        } else if parts.len() > 2 {
            return Ok(format_name(parts.get(0).unwrap(), parts.get(1).unwrap()))
        }
        return Err(format!("Could not parse email: {}", line))
    }
    else { Err(format!("Could not parse email: {}", line)) }
}

fn format_name(last_name: &str, first_name: &str) -> String {
    return format!("{initial}{last_name}", initial = first_name.get(0..1).unwrap(), last_name = last_name);
}

fn parse_email(line: &str) -> String {
    let position =  line.chars().position(|c| c == '@').unwrap();
    return line.chars().take(position).collect::<String>();
}

fn is_email(line: &str) -> bool {
    return line.chars().position(|c| c == '@').unwrap_or_default() > 0
}