pub mod calculator;

/// Formats a numeric result, dropping the fractional part for whole numbers.
pub fn format_result(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e15 {
        format!("{}", value as i64)
    } else {
        format!("{value}")
    }
}
