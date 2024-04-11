pub fn remove_char(s: &str) -> String {
    return s
        .chars()
        .skip(1)
        .take(&s.chars().count() - 2)
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .concat();
}

fn main() {
    let case = remove_char("eloquent");
    println!("{}", case);
}

#[cfg(test)]
mod tests {
    use super::remove_char;

    #[test]
    fn sample_cases() {
        assert_eq!(remove_char("eloquent"), "loquen");
        assert_eq!(remove_char("country"), "ountr");
        assert_eq!(remove_char("person"), "erso");
        assert_eq!(remove_char("place"), "lac");
        assert_eq!(remove_char("ok"), "");
        assert_eq!(remove_char("ooopsss"), "oopss");
    }
}
