mod antidupe {
    pub trait DirectoryContentsLister {

    }

    use std::path::Path;

    pub struct DuplicatedItem {
        path: Path
    }

    pub fn get_duplicates(lister: &DirectoryContentsLister) -> Vec<&DuplicatedItem> {
        Vec::new()
    }

}

#[cfg(test)]
mod antidupe_should_report {

    use antidupe::DirectoryContentsLister;
    use antidupe::get_duplicates;

    struct FakeDirectoryContentsLister { }

    impl FakeDirectoryContentsLister {
        fn new() -> FakeDirectoryContentsLister {
            FakeDirectoryContentsLister{}
        }
    }

    impl DirectoryContentsLister for FakeDirectoryContentsLister {}

    #[test]
    fn no_duplicates_for_empty_directory() {
        let lister = FakeDirectoryContentsLister::new();
        let duplicates = get_duplicates(&lister);
        assert_eq!(duplicates.len(), 0);
    }
}
