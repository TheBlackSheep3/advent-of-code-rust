struct Directory<'a> {
    files: Vec<&'a File>,
    directories: Vec<&'a Directory<'a>>,
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

    fn add_dir(&mut self, dir: &'a Directory) -> () {
        self.directories.push(dir);
    }

    fn clear_dirs(&mut self) -> () {
        self.directories.clear();
    }

    fn add_file(&mut self, file: &'a File) -> () {
        self.files.push(file);
    }

    fn clear_files(&mut self) -> () {
        self.files.clear();
    }
}

struct File {
    size: usize,
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

    #[test]
    fn get_size() {
        let mut root = Directory{files: Vec::new(), directories: Vec::new()};
        let f = File { size: usize::MAX};
        root.add_file(&f);
        assert_eq!(root.get_size(), Some(usize::MAX));
        root.add_file(&f);
        assert_eq!(root.get_size(), None);
        root.clear_files();
        assert_eq!(root.get_size(), Some(0));
        let mut dir1 = Directory{ files:Vec::new(), directories:Vec::new()};
        let f = File { size: 297 };
        dir1.add_file(&f);
        let f = File { size: 92 };
        dir1.add_file(&f);
        let mut dir2 = Directory{ files:Vec::new(), directories:Vec::new()};
        assert_eq!(dir1.get_size(), Some(389));
        let f = File { size: 201 };
        dir2.add_file(&f);
        let f = File { size: 927 };
        dir2.add_file(&f);
        assert_eq!(dir2.get_size(), Some(1128));
        dir1.add_dir(&dir2);
        root.add_dir(&dir1);
        assert_eq!(dir1.get_size(), Some(1517));
        assert_eq!(root.get_size(), Some(1517));
    }
}
