use super::file::File;

#[derive(Debug, PartialEq)]
pub struct DiskMap {
    blocks: Vec<Option<usize>>,
    files: Vec<File>,
}

impl std::str::FromStr for DiskMap {
    type Err = super::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        if lines.clone().count() == 1usize {
            let mut currently_file: bool = true;
            let mut file_id: usize = 0usize;
            let mut blocks: Vec<Option<usize>> = Vec::new();
            let mut files: Vec<File> = Vec::new();
            for char in lines.next().unwrap().chars() {
                if char.is_numeric() {
                    let size = char
                        .to_string()
                        .parse::<usize>()
                        .map_err(|_| super::Error::ParsingFailed)?;
                    if currently_file {
                        for _ in 0..size {
                            blocks.push(Some(file_id));
                        }
                        files.push(File { id: file_id, size });
                    } else {
                        for _ in 0..size {
                            blocks.push(None);
                        }
                        file_id = file_id.checked_add(1).ok_or(super::Error::ParsingFailed)?
                    }
                    currently_file = !currently_file;
                } else {
                    return Err(super::Error::ParsingFailed);
                }
            }
            Ok(DiskMap { blocks, files })
        } else {
            Err(super::Error::ParsingFailed)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DiskMap;
    use super::File;

    #[test]
    fn parse() {
        assert_eq!(
            super::super::tests::SMALL_SAMPLE1.parse::<DiskMap>(),
            Ok(DiskMap {
                blocks: vec![
                    vec![Some(0); 1],
                    vec![None; 2],
                    vec![Some(1); 3],
                    vec![None; 4],
                    vec![Some(2); 5]
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>(),
                files: vec![
                    File { id: 0, size: 1 },
                    File { id: 1, size: 3 },
                    File { id: 2, size: 5 }
                ]
            })
        );
        assert_eq!(
            super::super::tests::SMALL_SAMPLE2.parse::<DiskMap>(),
            Ok(DiskMap {
                blocks: vec![vec![Some(0); 9], vec![Some(1); 9], vec![Some(2); 9]]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>(),
                files: vec![
                    File { id: 0, size: 9 },
                    File { id: 1, size: 9 },
                    File { id: 2, size: 9 }
                ]
            })
        );
        assert_eq!(
            super::super::tests::LARGER_SAMPLE.parse::<DiskMap>(),
            Ok(DiskMap {
                blocks: vec![
                    vec![Some(0); 2],
                    vec![None; 3],
                    vec![Some(1); 3],
                    vec![None; 3],
                    vec![Some(2); 1],
                    vec![None; 3],
                    vec![Some(3); 3],
                    vec![None; 1],
                    vec![Some(4); 2],
                    vec![None; 1],
                    vec![Some(5); 4],
                    vec![None; 1],
                    vec![Some(6); 4],
                    vec![None; 1],
                    vec![Some(7); 3],
                    vec![None; 1],
                    vec![Some(8); 4],
                    vec![Some(9); 2]
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>(),
                files: vec![
                    File { id: 0, size: 2 },
                    File { id: 1, size: 3 },
                    File { id: 2, size: 1 },
                    File { id: 3, size: 3 },
                    File { id: 4, size: 2 },
                    File { id: 5, size: 4 },
                    File { id: 6, size: 4 },
                    File { id: 7, size: 3 },
                    File { id: 8, size: 4 },
                    File { id: 9, size: 2 },
                ]
            })
        );
        assert_eq!(
            "123\n456".parse::<DiskMap>(),
            Err(super::super::Error::ParsingFailed)
        );
        assert_eq!(
            "123\n".parse::<DiskMap>(),
            Ok(DiskMap {
                blocks: vec![vec![Some(0); 1], vec![None; 2], vec![Some(1); 3]]
                    .into_iter()
                    .flatten()
                    .collect(),
                files: vec![File { id: 0, size: 1 }, File { id: 1, size: 3 }]
            })
        );
        assert_eq!(
            "".parse::<DiskMap>(),
            Err(super::super::Error::ParsingFailed)
        );
        assert_eq!(
            "12d".parse::<DiskMap>(),
            Err(super::super::Error::ParsingFailed)
        );
    }
}
