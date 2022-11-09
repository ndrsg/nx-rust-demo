pub fn rust_lib_one() -> String {
    "rust-lib-one".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(rust_lib_one(), "rust-lib-one".to_string());
    }
}
