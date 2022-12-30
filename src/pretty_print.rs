#![forbid(unsafe_code)]

pub fn format_result(number: i128) -> String {
    if format!("{}", number).chars().count() < 5 {
        return format!("{}", number);
    }
    format!("{:.5e}", number)
}