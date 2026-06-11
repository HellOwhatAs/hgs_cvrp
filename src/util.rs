//! Small shared utilities and constants.

/// Precision parameter, used to avoid numerical instabilities (same as the C++ MY_EPSILON).
pub const MY_EPSILON: f64 = 0.00001;

/// Pi constant, kept identical to the C++ implementation for bit-compatible polar angles.
pub const PI: f64 = 3.14159265359;

/// Formats a float like the C++ default `std::ostream` (printf "%g" with 6 significant digits).
///
/// This keeps solution files and logs textually identical to the reference implementation.
pub fn format_double(value: f64) -> String {
    if value == 0.0 {
        return "0".to_string();
    }
    if value.is_nan() {
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value < 0.0 { "-inf" } else { "inf" }.to_string();
    }

    // Round to 6 significant digits first, then decide between fixed and scientific notation.
    let scientific = format!("{:.5e}", value);
    let (mantissa, exponent) = scientific
        .split_once('e')
        .expect("e-notation always has an exponent");
    let exponent: i32 = exponent.parse().expect("exponent is a valid integer");

    if exponent < -4 || exponent >= 6 {
        let mantissa = mantissa.trim_end_matches('0').trim_end_matches('.');
        let sign = if exponent < 0 { '-' } else { '+' };
        format!("{}e{}{:02}", mantissa, sign, exponent.abs())
    } else {
        let decimals = (5 - exponent).max(0) as usize;
        let fixed = format!("{:.*}", decimals, value);
        if fixed.contains('.') {
            fixed
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string()
        } else {
            fixed
        }
    }
}

#[cfg(test)]
mod tests {
    use super::format_double;

    #[test]
    fn formats_like_cpp_ostream() {
        assert_eq!(format_double(0.0), "0");
        assert_eq!(format_double(27591.0), "27591");
        assert_eq!(format_double(555.43), "555.43");
        assert_eq!(format_double(909.675), "909.675");
        assert_eq!(format_double(1077.5499999999999), "1077.55");
        assert_eq!(format_double(0.5), "0.5");
        assert_eq!(format_double(0.2), "0.2");
        assert_eq!(format_double(1e30), "1e+30");
        assert_eq!(format_double(999999.7), "1e+06");
        assert_eq!(format_double(123456.0), "123456");
        assert_eq!(format_double(-555.43), "-555.43");
        assert_eq!(format_double(0.000012345678), "1.23457e-05");
    }
}
