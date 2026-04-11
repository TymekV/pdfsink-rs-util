pub fn normalize(s: &str) -> String {
    s.chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalization_works() {
        assert_eq!(normalize("Lorem\nIpsum"), "loremipsum");
        assert_eq!(normalize("\n\n  Lorem \n Ipsum  \n"), "loremipsum");
    }
}
