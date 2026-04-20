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

        let upper_grouped = Self::group4(upper);
        let lower_grouped = Self::group4(lower);

        // total width of the grouped line
        let width = upper_grouped.len();

        // split into 3 logical sections
        let col = (width / 3) + 1;

        let header_top = format!("{:<col$}{:^col$}{:>col$}", "63", "47", "32", col = col);

        let header_bottom = format!("{:<col$}{:^col$}{:>col$}", "31", "15", "0", col = col);

        format!(
            "{upper}\n{top}\n\n{lower}\n{bottom}",
            upper = upper_grouped,
            top = header_top,
            lower = lower_grouped,
            bottom = header_bottom,
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
