use regex::Regex;

pub fn retrieve_deletable_dir_size(input: &str) -> Result<usize, FileStructureError> {
    let root: Directory = parse_file_structure(input)?;
    match root.get_sum_of_dirs_with_max_size(100_000) {
        Some(x) => Ok(x),
        None => Err(FileStructureError::Overflow),
    }
}

fn parse_file_structure(input: &str) -> Result<Directory, FileStructureError> {
    let directory_up_regex = Regex::new(r"\$ cd ..").unwrap();
    let change_directory_regex = Regex::new(r"\$ cd ([\w/]+)").unwrap();
    let file_regex = Regex::new(r"(\d+) [\w.]+").unwrap();
    let mut directory_stack: Vec<Directory> = Vec::new();
    for line in input.lines() {
        if directory_up_regex.is_match(line) {
            directory_stack.pop();
        } else if let Some(caps) = change_directory_regex.captures(line) {
            match caps.get(1) {
                None => return Err(FileStructureError::Parse(line)),
                Some(m) => {
                    let dir = Directory::new(m.as_str());
                    directory_stack.push(dir.clone());
                    if let Some(parent) = directory_stack.last_mut() {
                        parent.add_dir(dir.clone());
                    }
                }
            }
        } else if let Some(caps) = file_regex.captures(line) {
            match caps.get(1) {
                None => return Err(FileStructureError::Parse(line)),
                Some(size) => match directory_stack.last_mut() {
                    None => return Err(FileStructureError::MissingRootDirectory),
                    Some(dir) => dir.add_file(File {
                        size: usize::from_str_radix(size.as_str(), 10).unwrap(),
                    }),
                },
            }
        }
    }
    match directory_stack.first() {
        Some(dir) => Ok(dir.clone()),
        None => Err(FileStructureError::MissingRootDirectory),
    }
}

#[derive(PartialEq)]
pub enum FileStructureError<'a> {
    MissingRootDirectory,
    Parse(&'a str),
    Overflow,
}

impl<'a> std::fmt::Debug for FileStructureError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Overflow => write!(f, "integer overflow occured while calculating size"),
            Self::Parse(line) => write!(f, "failed to parse '{}'", line),
            Self::MissingRootDirectory => write!(f, "no root directory detected"),
        }
    }
}

impl<'a> std::fmt::Display for FileStructureError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, PartialEq)]
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

    fn add_dir(&mut self, dir: Directory<'a>) {
        self.directories.push(dir);
    }

    fn clear_dirs(&mut self) {
        self.directories.clear();
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    fn clear_files(&mut self) {
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

#[derive(Clone, Debug, PartialEq)]
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
    use rstest::rstest;

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
        let e = Directory {
            name: "e",
            files: vec![File { size: 584 }],
            directories: vec![],
        };
        let a = Directory {
            name: "a",
            files: vec![
                File { size: 29116 },
                File { size: 2557 },
                File { size: 62596 },
            ],
            directories: vec![Directory {
                name: "e",
                files: vec![File { size: 584 }],
                directories: vec![],
            }],
        };
        let d = Directory {
            name: "d",
            files: vec![
                File { size: 4060174 },
                File { size: 8033020 },
                File { size: 5626152 },
                File { size: 7214296 },
            ],
            directories: vec![],
        };
        let root = Directory {
            name: "/",
            files: vec![File { size: 14848514 }, File { size: 8504156 }],
            directories: vec![
                Directory {
                    name: "a",
                    files: vec![
                        File { size: 29116 },
                        File { size: 2557 },
                        File { size: 62596 },
                    ],
                    directories: vec![Directory {
                        name: "e",
                        files: vec![File { size: 584 }],
                        directories: vec![],
                    }],
                },
                Directory {
                    name: "d",
                    files: vec![
                        File { size: 4060174 },
                        File { size: 8033020 },
                        File { size: 5626152 },
                        File { size: 7214296 },
                    ],
                    directories: vec![],
                },
            ],
        };
        root
    }

    #[rstest]
    #[case(Directory{ name: "root", files: vec![], directories: vec![]}, Some(0))]
    #[case(Directory{ name: "root", files: vec![File { size: usize::MAX }], directories: vec![]}, Some(usize::MAX))]
    #[case(Directory{ name: "root", files: vec![File { size: usize::MAX }, File { size: usize::MAX }], directories: vec![] }, None)]
    #[case(Directory{ name: "1", files: vec![File { size: 297 }, File { size: 92 }], directories: vec![] }, Some(389))]
    #[case(Directory{ name: "2", files: vec![File { size: 201 }, File { size: 927 }], directories: vec![] }, Some(1128))]
    #[case(Directory{ name: "1", files: vec![File { size: 297 }, File { size: 92 }], directories: vec![ Directory{ name: "2", files: vec![File { size: 201 }, File { size: 927 } ], directories: vec![] }] }, Some(1517))]
    #[case(Directory{ name: "root", files: vec![], directories: vec![ Directory{ name: "1", files: vec![File { size: 297 }, File { size: 92 }], directories: vec![ Directory{ name: "2", files: vec![File { size: 201 }, File { size: 927 } ], directories: vec![] }] } ]}, Some(1517))]
    fn get_size(#[case] directory: Directory, #[case] expected: Option<usize>) {
        assert_eq!(expected, directory.get_size())
    }

    // TODO: reenable test and get it to succeed
    #[ignore = "implementation isn't ready yet"]
    #[test]
    fn parse() {
        assert_eq!(
            parse_file_structure(TEST_INPUT),
            Ok(get_parsed_test_input())
        );
    }

    #[rstest]
    #[case(Directory { name: "/", files: vec![File { size: 14848514 }, File { size: 8504156 }], directories: vec![ Directory { name: "a", files: vec![ File { size: 29116 }, File { size: 2557 }, File { size: 62596 }, ], directories: vec![Directory { name: "e", files: vec![File { size: 584 }], directories: vec![], }], }, Directory { name: "d", files: vec![ File { size: 4060174 }, File { size: 8033020 }, File { size: 5626152 }, File { size: 7214296 }, ], directories: vec![], }, ], }, 100_000, Some(95437))]
    #[case(Directory { name: "a", files: vec![ File { size: 29116 }, File { size: 2557 }, File { size: 62596 }, ], directories: vec![Directory { name: "e", files: vec![File { size: 584 }], directories: vec![], }], }, 100_000, Some(95437))]
    #[case(Directory { name: "e", files: vec![File { size: 584 }], directories: vec![], }, 100_000, Some(584))]
    #[case(Directory { name: "d", files: vec![ File { size: 4060174 }, File { size: 8033020 }, File { size: 5626152 }, File { size: 7214296 }, ], directories: vec![], }, 100_000, Some(0))]
    fn get_size_maxsum(
        #[case] directory: Directory,
        #[case] max_size: usize,
        #[case] expected: Option<usize>,
    ) {
        assert_eq!(expected, directory.get_sum_of_dirs_with_max_size(max_size));
    }
}
