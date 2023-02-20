pub fn retrieve_deletable_dir_size(input: &str) -> Result<usize, FileStructureError> {
    let root: Directory = parse_file_structure(input)?;
    match root.get_sum_of_dirs_with_max_size(100_000) {
        Some(x) => Ok(x),
        None => Err(FileStructureError::Overflow),
    }
}

fn parse_file_structure(input: &str) -> Result<Directory, FileStructureError> {
    let mut directory_stack: Vec<Directory> = Vec::new();
    for line in input.lines() {}
    match directory_stack.first() {
        Some(dir) => Ok(dir.clone()),
        None => Err(FileStructureError::MissingRootDirectory),
    }
}

pub enum FileStructureError<'a> {
    MissingRootDirectory,
    Parse(&'a str),
    Overflow,
}

impl<'a> std::fmt::Display for FileStructureError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Overflow => write!(f, "integer overflow occured while calculating size"),
            Self::Parse(line) => write!(f, "failed to parse '{}'", line),
            Self::MissingRootDirectory => write!(f, "no root directory detected"),
        }
    }
}

#[derive(Clone)]
struct Directory<'a> {
    name: &'a str,
    files: Vec<File>,
    directories: Vec<Directory<'a>>,
}

impl<'a> Directory<'a> {
    fn get_size(&self) -> Option<usize> {
        let mut sum: usize = 0;
        for dir in &self.directories {
            sum = sum.checked_add(dir.get_size()?)?;
        }
        for file in &self.files {
            sum = sum.checked_add(file.size)?;
        }
        Some(sum)
    }

    fn add_dir(&mut self, dir: Directory<'a>) -> () {
        self.directories.push(dir);
    }

    fn clear_dirs(&mut self) -> () {
        self.directories.clear();
    }

    fn add_file(&mut self, file: File) -> () {
        self.files.push(file);
    }

    fn clear_files(&mut self) -> () {
        self.files.clear();
    }

    fn get_child_by_name(&self, name: &'a str) -> Option<&Directory> {
        let mut tmp = self.directories.iter().filter(|dir| dir.name == name);
        tmp.next()
    }

    fn get_sum_of_dirs_with_max_size(&self, max_size: usize) -> Option<usize> {
        let mut sum: usize = 0;
        for dir in &self.directories {
            sum = sum.checked_add(dir.get_sum_of_dirs_with_max_size(max_size)?)?;
        }
        let own_size = self.get_size()?;
        if own_size <= max_size {
            sum = sum.checked_add(own_size)?;
        }
        Some(sum)
    }

    const fn new(name: &'a str) -> Directory<'a> {
        Directory {
            name,
            files: Vec::new(),
            directories: Vec::new(),
        }
    }
}

#[derive(Clone)]
struct File {
    size: usize,
}

impl File {
    const fn new(size: usize) -> File {
        File { size }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    fn get_parsed_test_input() -> Directory<'static> {
        let mut e = Directory::new("e");
        let i = File::new(584);
        e.add_file(i);
        let mut a = Directory::new("a");
        a.add_dir(e);
        let f = File::new(29116);
        a.add_file(f);
        let g = File::new(2557);
        a.add_file(g);
        let h = File::new(62596);
        a.add_file(h);
        let mut d = Directory::new("d");
        let j = File::new(4060174);
        d.add_file(j);
        let dlog = File::new(8033020);
        d.add_file(dlog);
        let dext = File::new(5626152);
        d.add_file(dext);
        let k = File::new(7214296);
        d.add_file(k);
        let mut root = Directory::new("/");
        root.add_dir(a);
        root.add_dir(d);
        let b = File::new(14848514);
        root.add_file(b);
        let c = File::new(8504156);
        root.add_file(c);
        root
    }

    #[test]
    fn get_size() {
        let mut root = Directory::new("root");
        let f = File { size: usize::MAX };
        root.add_file(f.clone());
        assert_eq!(root.get_size(), Some(usize::MAX));
        root.add_file(f);
        assert_eq!(root.get_size(), None);
        root.clear_files();
        assert_eq!(root.get_size(), Some(0));
        let mut dir1 = Directory::new("1");
        let f = File { size: 297 };
        dir1.add_file(f);
        let f = File { size: 92 };
        dir1.add_file(f);
        let mut dir2 = Directory::new("2");
        assert_eq!(dir1.get_size(), Some(389));
        let f = File { size: 201 };
        dir2.add_file(f);
        let f = File { size: 927 };
        dir2.add_file(f);
        assert_eq!(dir2.get_size(), Some(1128));
        dir1.add_dir(dir2);
        assert_eq!(dir1.get_size(), Some(1517));
        root.add_dir(dir1);
        assert_eq!(root.get_size(), Some(1517));
    }

    #[test]
    fn get_size_maxsum() {
        const MAX_SIZE: usize = 100_000;
        let root = get_parsed_test_input();
        let a = root.get_child_by_name("a").unwrap();
        let e = a.get_child_by_name("e").unwrap();
        let d = root.get_child_by_name("d").unwrap();
        assert_eq!(e.get_sum_of_dirs_with_max_size(MAX_SIZE), Some(584));
        assert_eq!(a.get_sum_of_dirs_with_max_size(MAX_SIZE), Some(95437));
        assert_eq!(d.get_sum_of_dirs_with_max_size(MAX_SIZE), Some(0));
        assert_eq!(root.get_sum_of_dirs_with_max_size(MAX_SIZE), Some(95437));
    }
}
