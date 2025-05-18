#[derive(Debug)]
pub struct FileStatistics {
    pub bytes: usize,
    pub words: usize,
    pub lines: usize,
    pub chars: usize,
}

impl FileStatistics {
    pub fn get_string_statistics(content: String) -> Result<FileStatistics, std::io::Error> {
        let bytes = content.len();
        let words = content.split_whitespace().count();
        let lines = content.lines().count();
        let chars = content.chars().count();

        Ok(FileStatistics {
            bytes,
            words,
            lines,
            chars,
        })
    }
}
