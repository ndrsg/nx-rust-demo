pub fn rust_lib_two() -> String {
    "rust-lib-two".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(rust_lib_two(), "rust-lib-two".to_string());
    }
}
