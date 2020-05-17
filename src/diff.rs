pub struct DirDiff {
    /// directory to compare
    pub dir: String,
    /// comparation directory
    pub dir_comp: String,
    /// exclude directories or files from the comparation
    pub excluding: Option<Vec<String>>,
    /// exclude recursively or only the from the root path
    pub recursive_excluding: bool,
}

pub struct FileDiff {
    /// file to compare
    pub file: String,
    /// comparation file
    pub file_comp: String,
}
