mod antidupe {
    pub trait DirectoryContentsLister {
        
    }

    use std::path::Path;

    pub struct DuplicatedItem {
        path: Path
    }

    use std::iter::empty;

    pub fn get_duplicates<'a>(lister: &'a DirectoryContentsLister) -> Iterator<Item = &'a DuplicatedItem> {
        empty::<&DuplicatedItem>();
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
        assert_eq!(duplicates.count(), 0);
    }
}
