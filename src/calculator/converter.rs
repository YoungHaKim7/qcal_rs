pub struct Converter;

impl Converter {
    pub fn unicode(input: &str) -> String {
        input
            .trim_matches(|c| c == '"' || c == '\'')
            .chars()
            .enumerate()
            .map(|(i, c)| format!("[{}] '{}' → U+{:04X}", i, c, c as u32))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
