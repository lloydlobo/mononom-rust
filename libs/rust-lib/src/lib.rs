pub fn rust_lib() -> String {
    "rust_lib".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(rust_lib(), "rust_lib".to_string());
    }
}
