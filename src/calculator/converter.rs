pub struct Converter;

impl Converter {
    pub fn unicode(input: &str) -> String {
        // 큰따옴표로 둘러싸인 문자열 찾기
        if let Some(start) = input.find('"')
            && let Some(end) = input[start + 1..].find('"')
        {
            let content = &input[start + 1..start + 1 + end];
            return content
                .chars()
                .enumerate()
                .map(|(i, c)| format!("[{}] '{}' → U+{:04X}", i, c, c as u32))
                .collect::<Vec<_>>()
                .join("\n");
        }

        // 큰따옴표가 없으면 전체 입력 처리
        input
            .chars()
            .enumerate()
            .map(|(i, c)| format!("[{}] '{}' → U+{:04X}", i, c, c as u32))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
