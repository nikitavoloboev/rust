use mdka::from_html;
use reqwest::blocking::get;

pub fn html_from_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = get(url)?;
    let html = response.text()?;
    let markdown = from_html(&html);
    log!(markdown);
    log!("---");
    let clean_markdown = clean_markdown(&markdown);
    log!(clean_markdown);
    log!("---");
    Ok(markdown)
}

pub fn clean_markdown(markdown: &str) -> String {
    let mut cleaned_lines = Vec::new();

    for line in markdown.lines() {
        let trimmed_line = line.trim();

        // Skip empty lines and lines starting with certain characters
        if trimmed_line.is_empty()
            || trimmed_line.starts_with('#')
            || trimmed_line.starts_with('-')
            || trimmed_line.starts_with('|')
            || trimmed_line.starts_with('>')
            || trimmed_line.starts_with('[')
            || trimmed_line.starts_with('<')
            || trimmed_line.starts_with('`')
        {
            continue;
        }

        // Remove any remaining Markdown formatting
        let cleaned_line = trimmed_line
            .replace("**", "")
            .replace("__", "")
            .replace("*", "")
            .replace("_", "")
            .replace("`", "")
            .replace("~", "")
            .replace(":", "")
            .replace("|", "")
            .replace("[", "")
            .replace("]", "")
            .replace("(", "")
            .replace(")", "")
            .replace("<", "")
            .replace(">", "")
            .trim()
            .to_string();

        // Add the cleaned line to the vector if it's not empty
        if !cleaned_line.is_empty() {
            cleaned_lines.push(cleaned_line);
        }
    }

    // Join the cleaned lines into a single string
    cleaned_lines.join(" ")
}
