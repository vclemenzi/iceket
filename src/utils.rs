pub fn is_url(text: &str) -> bool {
    if text.to_string().starts_with("http://") || text.to_string().starts_with("https://") {
        return true;
    } else {
        return false;
    }
}