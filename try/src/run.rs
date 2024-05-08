use mdka::from_html;
use reqwest::blocking::get;

// let url = "https://github.com/teamhanko/hanko/blob/main/frontend/elements/README.md";
pub fn fetch_and_print_markdown_from_url(url: &str) {
    match html_from_url(url) {
        Ok(markdown) => {
            println!("{}", markdown);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

pub fn html_from_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = get(url)?;
    let html = response.text()?;
    let markdown = from_html(&html);
    log!(markdown);
    log!("---");
    let clean_markdown = clean_markdown(&markdown);
    log!(clean_markdown);
    log!("---");
    Ok(clean_markdown)
}

pub fn clean_markdown(markdown: &str) -> String {
    let mut cleaned_lines = Vec::new();

    for line in markdown.lines() {
        let mut cleaned_line = line.trim().to_string();

        // Skip empty lines and lines starting with certain characters
        if cleaned_line.is_empty()
            || cleaned_line.starts_with('#')
            || cleaned_line.starts_with('-')
            || cleaned_line.starts_with('|')
            || cleaned_line.starts_with('>')
            || cleaned_line.starts_with('[')
            || cleaned_line.starts_with('<')
            || cleaned_line.starts_with('`')
        {
            continue;
        }

        // Remove any remaining Markdown formatting
        cleaned_line = cleaned_line.replace("**", "");
        cleaned_line = cleaned_line.replace("__", "");
        cleaned_line = cleaned_line.replace("*", "");
        cleaned_line = cleaned_line.replace("_", "");
        cleaned_line = cleaned_line.replace("`", "");
        cleaned_line = cleaned_line.replace("~", "");
        cleaned_line = cleaned_line.replace(":", "");
        cleaned_line = cleaned_line.replace("|", "");
        cleaned_line = cleaned_line.replace("[", "");
        cleaned_line = cleaned_line.replace("]", "");
        cleaned_line = cleaned_line.replace("(", "");
        cleaned_line = cleaned_line.replace(")", "");
        cleaned_line = cleaned_line.replace("<", "");
        cleaned_line = cleaned_line.replace(">", "");
        cleaned_line = cleaned_line.trim().to_string();

        // Skip lines that contain only special characters or numbers
        if !cleaned_line.chars().any(|c| c.is_alphabetic()) {
            continue;
        }

        // Remove consecutive whitespace and replace with a single space
        cleaned_line = cleaned_line
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");

        cleaned_lines.push(cleaned_line);
    }

    // Join the cleaned lines into a single string
    cleaned_lines.join(" ")
}
