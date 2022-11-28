use std::error::Error;

pub fn translate(source: String, dest: String, text: String) -> Result<String, Box<dyn Error>> {
    let escaped_text = text.replace("/", "[slash]").replace("?", "[question_mark]");
    let request = format!("https://lingva.ml/api/v1/{source}/{dest}/{escaped_text}");

    let resp = reqwest::blocking::get(request)?.text()?;

    let parsed = json::parse(&resp).unwrap();
    if parsed.has_key("translation") {
        let result = parsed["translation"].dump();
        return Ok(result[1 .. result.len() - 1].to_string().replace("[slash]", "/").replace("[question_mark]", "?"));
    }

    Ok(resp)
}
