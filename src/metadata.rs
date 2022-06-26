use chrono::Utc;
use serde_json::json;

pub fn format_metadata(author: Option<String>, readme: Option<String>) -> String {
    let author = author.unwrap_or_else(|| "Anonymous".to_string());
    let readme = readme.unwrap_or_else(|| "Exported by asmjr".to_string());
    let exportdate = Utc::now().to_string();

    json!({
      "author": author,
      "date": exportdate,
      "readme": readme,
      "toolchain": "asmjr"
    })
    .to_string()
}
