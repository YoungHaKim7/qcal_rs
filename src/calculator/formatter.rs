use crate::fprice::PriceFormatter;

pub struct Formatter;

impl Formatter {
    pub fn full(value: f64) -> String {
        let int_value = value as i64;
        let dec = PriceFormatter::format(int_value);

        // Format the floating-point value, removing unnecessary trailing zeros
        let float_str = if value.fract() == 0.0 {
            format!("{}", value as i64)
        } else {
            // Remove trailing zeros but keep at least one decimal place
            let s = format!("{:.6}", value);
            s.trim_end_matches('0').trim_end_matches('.').to_string()
        };

        format!(
            "        {}\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━\nHEX : \"0x{:X}\"\nDEC : \"{}\"\nOCT : \"0o{:o}\"\nBIN : \"{}\"\n{}\n",
            float_str,
            int_value,
            dec,
            int_value,
            Self::bin(int_value),
            Self::format_64bit(int_value)
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

    fn format_64bit(value: i64) -> String {
        let bits = format!("{:064b}", value);

        let upper = &bits[0..32];
        let lower = &bits[32..64];

        format!(
            "{}\n63                      47                  32\n\n{}\n31                      15                   0",
            Self::group4(upper),
            Self::group4(lower)
        )
    }

    fn group4(s: &str) -> String {
        s.chars()
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("  ")
    }
}
