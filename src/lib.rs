mod antidupe {

    use std::path::Path;

    pub struct DuplicatedItem {
    }

    pub fn duplicates(path: &Path) -> Vec<&DuplicatedItem> {
        Vec::new()
    }

}

#[cfg(test)]
mod antidupe_should_report {
    extern crate tempdir;

    use self::tempdir::TempDir;
    use antidupe::duplicates;

    #[test]
    fn no_duplicates_for_empty_directory() {
        let empty_directory = TempDir::new("empty").unwrap();
        let duplicates = duplicates(&empty_directory.path());
        assert_eq!(duplicates.len(), 0);
    }
}
