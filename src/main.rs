fn main() {
    println!("Hello, world!");
}


mod antidupe {
    use std::path::Path;

    trait InputPath {}

    impl InputPath for Path {}

    #[cfg(test)]
    mod antidupe_should_return {
        #[test]
        fn no_duplicates_for_empty_directory() {}
    }
}