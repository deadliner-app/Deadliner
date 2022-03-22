pub fn is_string_numeric(word: &str) -> bool {
    for c in word.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}
