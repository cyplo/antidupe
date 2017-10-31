mod antidupe {

    pub struct DirectoryEntry {

    }

    pub trait DirectoryContentsLister {
        fn entries(&self) -> Vec<&DirectoryEntry>;
    }

    pub struct DuplicatedItem {
    }

    pub fn duplicates(lister: &DirectoryContentsLister) -> Vec<&DuplicatedItem> {
        Vec::new()
    }

}

#[cfg(test)]
mod antidupe_should_report {

    use antidupe::DirectoryEntry;
    use antidupe::DirectoryContentsLister;
    use antidupe::duplicates;

    struct FakeDirectoryContentsLister {
        entries: Vec<DirectoryEntry>
    }

    impl FakeDirectoryContentsLister {
        fn empty() -> FakeDirectoryContentsLister {
            FakeDirectoryContentsLister{ entries: Vec::new() }
        }
        fn single_file() -> FakeDirectoryContentsLister {
            FakeDirectoryContentsLister{
                entries: vec![],
            }
        }
    }

    impl DirectoryContentsLister for FakeDirectoryContentsLister {
        fn entries(&self) -> Vec<&DirectoryEntry> {
            unimplemented!()
        }
    }

    #[test]
    fn no_duplicates_for_empty_directory() {
        let lister = FakeDirectoryContentsLister::empty();
        let duplicates = duplicates(&lister);
        assert_eq!(duplicates.len(), 0);
    }

    #[test]
    fn no_duplicates_for_directory_with_one_file() {
        let lister = FakeDirectoryContentsLister::single_file();
        let duplicates = duplicates(&lister);
        assert_eq!(duplicates.len(), 0);
    }
}
