use crate::fprice::PriceFormatter;

pub struct Formatter;

impl Formatter {
    pub fn full(value: i64) -> String {
        let dec = PriceFormatter::format(value);

        format!(
            "        {}\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━\nHEX : \"0x{:X}\"\nDEC : \"{}\"\nOCT : \"0o{:o}\"\nBIN : \"{}\"",
            dec,
            value,
            dec,
            value,
            Self::bin(value)
        )
    }

    fn bin(v: i64) -> String {
        let raw = format!("{:b}", v);
        let pad = (4 - raw.len() % 4) % 4;
        let s = format!("{}{}", "0".repeat(pad), raw);

        s.chars()
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join(" ")
    }
}
