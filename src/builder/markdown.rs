use async_std::{fs, path::Path};
use pulldown_cmark::{html, Options, Parser};

/// `markdown()` is the public-facing generic function that wraps around `parse_markdown()`
pub async fn markdown<P: AsRef<Path>>(path: &P, entry: &fs::DirEntry) -> Result<(), String> {
    match entry.path().to_str() {
        Some(name) => match parse_markdown(path.as_ref()).await {
            Ok(raw) => super::utils::write_string_to_file(&raw, name, "html").await,
            Err(e) => Err(e.to_string()),
        },
        None => Err(format!("unable to parse file")),
    }
}
/// `parse_markdown()` opens
async fn parse_markdown(path: &Path) -> Result<String, &str> {
    let mut html_output = String::new();
    if let Ok(raw) = fs::read_to_string(path).await {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
        let parser = Parser::new_ext(&raw, options);
        html::push_html(&mut html_output, parser);
    }
    Ok(html_output)
}
