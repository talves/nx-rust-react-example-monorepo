pub fn rust_test_lib() -> String {
    "rust_test_lib".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(rust_test_lib(), "rust_test_lib".to_string());
    }
}
