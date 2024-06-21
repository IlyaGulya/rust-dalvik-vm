pub fn is_debug() -> bool {
    return std::env::var("DEBUG_MODE")
        .map(|v| v == "true")
        .unwrap_or(false);
}