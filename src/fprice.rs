#[derive(Debug)]
pub struct PriceComma<T> {
    data: T,
    result: String,
}

trait Formatter {
    fn fmt_number(&self) -> String;
    fn big_num_i64_str(&self) -> String;
    fn fmt_num_f64_str(&self) -> String;
    fn fmt_num_str(&self) -> String;
}

impl<T> PriceComma<T> {
    fn new(data: T, result: String) -> Self {
        Self { data, result }
    }
    fn push(&mut self, data: T) {
        self.data = data;
    }
}

impl<T: Default> Default for PriceComma<T> {
    fn default() -> Self {
        Self::new(T::default(), String::new())
    }
}

impl<T: ToString> Formatter for PriceComma<T> {
    // i32 fn
    fn fmt_number(&self) -> String {
        let mut formatted_number = String::new();
        let mut count = 0;
        let number_convert = self.data.to_string();

        // Handle negative numbers
        let (num_str, is_negative) = if number_convert.starts_with('-') {
            (&number_convert[1..], true)
        } else {
            (number_convert.as_str(), false)
        };

        // Iterate through the characters of the number from right to left
        for c in num_str.chars().rev() {
            if count == 3 {
                formatted_number.push(',');
                count = 0;
            }
            formatted_number.push(c);
            count += 1;
        }

        // Reverse the formatted number back to the original order
        let mut result = formatted_number.chars().rev().collect::<String>();

        // Add negative sign back if needed
        if is_negative {
            result.insert(0, '-');
        }

        result
    }

    // big number
    fn big_num_i64_str(&self) -> String {
        let mut formatted_number = String::new();
        let mut count = 0;
        let number_convert = self.data.to_string();

        // Handle negative numbers
        let (num_str, is_negative) = if number_convert.starts_with('-') {
            (&number_convert[1..], true)
        } else {
            (number_convert.as_str(), false)
        };

        // Iterate through the characters of the number from right to left
        for c in num_str.chars().rev() {
            if count == 3 {
                formatted_number.push(',');
                count = 0;
            }
            formatted_number.push(c);
            count += 1;
        }

        // Reverse the formatted number back to the original order
        let mut result = formatted_number.chars().rev().collect::<String>();

        // Add negative sign back if needed
        if is_negative {
            result.insert(0, '-');
        }

        result
    }

    // 123.45 소수점 콤마 표시fn
    fn fmt_num_f64_str(&self) -> String {
        let mut formatted_number = String::new();
        let mut count = 0;
        let number_convert = self.data.to_string();
        let mut number_point_raw = number_convert.split('.');

        let integer_part = number_point_raw.next().unwrap();
        let decimal_part = number_point_raw.next().unwrap_or("00");

        // Handle negative numbers
        let (int_str, is_negative) = if integer_part.starts_with('-') {
            (&integer_part[1..], true)
        } else {
            (integer_part, false)
        };

        // Iterate through the characters of the integer part from right to left
        for c in int_str.chars().rev() {
            if count == 3 {
                formatted_number.push(',');
                count = 0;
            }
            formatted_number.push(c);
            count += 1;
        }

        // Reverse the formatted number back to the original order
        let mut result = formatted_number.chars().rev().collect::<String>();

        // Add negative sign back if needed
        if is_negative {
            result.insert(0, '-');
        }

        // Combine the integer and decimal parts
        result.push('.');
        result.push_str(decimal_part);

        result
    }

    // "123" String 숫자 콤마 표시fn
    fn fmt_num_str(&self) -> String {
        let number = self.data.to_string();
        let mut formatted_number = String::new();
        let mut count = 0;

        // Handle negative numbers
        let (num_str, is_negative) = if number.starts_with('-') {
            (&number[1..], true)
        } else {
            (number.as_str(), false)
        };

        // Iterate through the characters of the number from right to left
        for c in num_str.chars().rev() {
            if count == 3 {
                formatted_number.push(',');
                count = 0;
            }
            formatted_number.push(c);
            count += 1;
        }

        // Reverse the formatted number back to the original order
        let mut result = formatted_number.chars().rev().collect::<String>();

        // Add negative sign back if needed
        if is_negative {
            result.insert(0, '-');
        }

        result
    }
}

/// Format a number with comma separators every 3 digits.
///
/// # Examples
///
/// ```
/// use tcal_rs::fprice::fprice;
///
/// assert_eq!(fprice(1234567), "1,234,567");
/// assert_eq!(fprice(-1234567), "-1,234,567");
/// assert_eq!(fprice(123), "123");
/// ```
pub fn fprice<T: ToString>(value: T) -> String {
    let pc = PriceComma::new(value, String::new());
    pc.fmt_number()
}

/// Format a floating-point number with comma separators.
///
/// # Examples
///
/// ```
/// use tcal_rs::fprice::fprice_float;
///
/// assert_eq!(fprice_float(1234567.89), "1,234,567.89");
/// assert_eq!(fprice_float(-1234567.89), "-1,234,567.89");
/// ```
pub fn fprice_float<T: ToString>(value: T) -> String {
    let pc = PriceComma::new(value, String::new());
    pc.fmt_num_f64_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_numbers() {
        assert_eq!(fprice(123), "123");
        assert_eq!(fprice(1234), "1,234");
        assert_eq!(fprice(1234567), "1,234,567");
        assert_eq!(fprice(1234567890), "1,234,567,890");
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(fprice(-123), "-123");
        assert_eq!(fprice(-1234), "-1,234");
        assert_eq!(fprice(-1234567), "-1,234,567");
        assert_eq!(fprice(-1234567890), "-1,234,567,890");
    }

    #[test]
    fn test_fprice_float() {
        assert_eq!(fprice_float(123.45), "123.45");
        assert_eq!(fprice_float(1234.56), "1,234.56");
        assert_eq!(fprice_float(1234567.89), "1,234,567.89");
    }

    #[test]
    fn test_fprice_float_negative() {
        assert_eq!(fprice_float(-123.45), "-123.45");
        assert_eq!(fprice_float(-1234.56), "-1,234.56");
        assert_eq!(fprice_float(-1234567.89), "-1,234,567.89");
    }
}
