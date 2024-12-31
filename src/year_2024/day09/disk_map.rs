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

impl DiskMap {
    pub fn rearrange(&mut self) {
        let mut free_space_index: usize = 0;
        let mut occupied_space_index: usize = self.blocks.len() - 1;
        while free_space_index < occupied_space_index {
            match (
                self.blocks[free_space_index],
                self.blocks[occupied_space_index],
            ) {
                (Some(_), None) => {
                    free_space_index += 1;
                    occupied_space_index -= 1;
                }
                (Some(_), Some(_)) => {
                    free_space_index += 1;
                }
                (None, None) => {
                    occupied_space_index -= 1;
                }
                (a, b) => {
                    self.blocks[free_space_index] = b;
                    self.blocks[occupied_space_index] = a;
                    free_space_index += 1;
                    occupied_space_index -= 1;
                }
            }
        }
    }

    pub fn rearrange_no_fragmentation(&mut self) {
        for file in self.files.iter().rev() {
            match (
                self.blocks
                    .windows(file.size)
                    .position(|w| w.iter().all(|b| *b == None)),
                self.blocks.iter().position(|b| *b == Some(file.id)),
            ) {
                (Some(position_empty), Some(position_file)) => {
                    if position_empty < position_file {
                        let file_data = self.blocks[position_file];
                        for i in 0..file.size {
                            self.blocks[position_empty + i] = file_data;
                            self.blocks[position_file + i] = None;
                        }
                    }
                }
                _ => (),
            }
        }
    }

    pub fn get_check_sum(&self) -> Result<usize, super::Error> {
        let mut checksum: usize = 0;
        for (position, block) in self.blocks.iter().enumerate() {
            match block {
                Some(file_id) => {
                    checksum = position
                        .checked_mul(*file_id)
                        .and_then(|x| x.checked_add(checksum))
                        .ok_or(super::Error::IntegerOverflow)?;
                }
                None => (),
            }
        }
        Ok(checksum)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::super::tests::LARGER_SAMPLE;
    use super::super::tests::SMALL_SAMPLE1;
    use super::super::tests::SMALL_SAMPLE2;
    use super::super::Error;
    use super::DiskMap;
    use super::File;

    #[rstest]
    #[case(SMALL_SAMPLE1, Ok(DiskMap { blocks: vec![ vec![Some(0); 1], vec![None; 2], vec![Some(1); 3], vec![None; 4], vec![Some(2); 5] ] .into_iter() .flatten() .collect::<Vec<_>>(), files: vec![ File { id: 0, size: 1 }, File { id: 1, size: 3 }, File { id: 2, size: 5 } ] }))]
    #[case(SMALL_SAMPLE2, Ok(DiskMap { blocks: vec![vec![Some(0); 9], vec![Some(1); 9], vec![Some(2); 9]] .into_iter() .flatten() .collect::<Vec<_>>(), files: vec![ File { id: 0, size: 9 }, File { id: 1, size: 9 }, File { id: 2, size: 9 } ] }))]
    #[case(LARGER_SAMPLE, Ok(DiskMap { blocks: vec![ vec![Some(0); 2], vec![None; 3], vec![Some(1); 3], vec![None; 3], vec![Some(2); 1], vec![None; 3], vec![Some(3); 3], vec![None; 1], vec![Some(4); 2], vec![None; 1], vec![Some(5); 4], vec![None; 1], vec![Some(6); 4], vec![None; 1], vec![Some(7); 3], vec![None; 1], vec![Some(8); 4], vec![Some(9); 2] ] .into_iter() .flatten() .collect::<Vec<_>>(), files: vec![ File { id: 0, size: 2 }, File { id: 1, size: 3 }, File { id: 2, size: 1 }, File { id: 3, size: 3 }, File { id: 4, size: 2 }, File { id: 5, size: 4 }, File { id: 6, size: 4 }, File { id: 7, size: 3 }, File { id: 8, size: 4 }, File { id: 9, size: 2 }, ] }))]
    #[case("123\n456", Err(Error::ParsingFailed))]
    #[case("123\n", Ok(DiskMap { blocks: vec![vec![Some(0); 1], vec![None; 2], vec![Some(1); 3]] .into_iter() .flatten() .collect(), files: vec![File { id: 0, size: 1 }, File { id: 1, size: 3 }] }))]
    #[case("", Err(Error::ParsingFailed))]
    #[case("12d", Err(Error::ParsingFailed))]
    fn parse(#[case] input: &str, #[case] expected: Result<DiskMap, Error>) {
        assert_eq!(expected, input.parse::<DiskMap>())
    }

    #[rstest]
    #[case(SMALL_SAMPLE1.parse().unwrap(), DiskMap { blocks: vec![ vec![Some(0); 1], vec![Some(2); 2], vec![Some(1); 3], vec![Some(2); 3], vec![None; 6], ] .into_iter() .flatten() .collect(), files: vec![ File { id: 0, size: 1 }, File { id: 1, size: 3 }, File { id: 2, size: 5 }, ], },)]
    #[case(SMALL_SAMPLE2.parse().unwrap(), SMALL_SAMPLE2.parse().unwrap())]
    #[case(LARGER_SAMPLE.parse().unwrap(), DiskMap { blocks: vec![ vec![Some(0); 2], vec![Some(9); 2], vec![Some(8); 1], vec![Some(1); 3], vec![Some(8); 3], vec![Some(2); 1], vec![Some(7); 3], vec![Some(3); 3], vec![Some(6); 1], vec![Some(4); 2], vec![Some(6); 1], vec![Some(5); 4], vec![Some(6); 2], vec![None; 14], ] .into_iter() .flatten() .collect(), files: vec![ File { id: 0, size: 2 }, File { id: 1, size: 3 }, File { id: 2, size: 1 }, File { id: 3, size: 3 }, File { id: 4, size: 2 }, File { id: 5, size: 4 }, File { id: 6, size: 4 }, File { id: 7, size: 3 }, File { id: 8, size: 4 }, File { id: 9, size: 2 }, ], },)]
    fn rearrange(#[case] input: DiskMap, #[case] expected: DiskMap) {
        let mut modifiable_map = input;
        modifiable_map.rearrange();
        assert_eq!(expected.files.len(), modifiable_map.files.len());
        assert_eq!(expected, modifiable_map);
    }

    #[rstest]
    #[case(SMALL_SAMPLE2.parse().unwrap(), SMALL_SAMPLE2.parse().unwrap())]
    #[case(LARGER_SAMPLE.parse().unwrap(), DiskMap { blocks: vec![ vec![Some(0); 2], vec![Some(9); 2], vec![Some(2); 1], vec![Some(1); 3], vec![Some(7); 3], vec![None; 1], vec![Some(4); 2], vec![None; 1], vec![Some(3); 3], vec![None; 4], vec![Some(5); 4], vec![None; 1], vec![Some(6); 4], vec![None; 5], vec![Some(8); 4], vec![None; 2], ] .into_iter() .flatten() .collect(), files: vec![ File { id: 0, size: 2 }, File { id: 1, size: 3 }, File { id: 2, size: 1 }, File { id: 3, size: 3 }, File { id: 4, size: 2 }, File { id: 5, size: 4 }, File { id: 6, size: 4 }, File { id: 7, size: 3 }, File { id: 8, size: 4 }, File { id: 9, size: 2 }, ], })]
    fn rearrange_no_fragmentation(#[case] input: DiskMap, #[case] expected: DiskMap) {
        let mut modifiable_map = input;
        modifiable_map.rearrange_no_fragmentation();
        assert_eq!(expected.files.len(), modifiable_map.files.len());
        assert_eq!(expected, modifiable_map);
    }

    #[rstest]
    #[case(SMALL_SAMPLE1.parse().unwrap(), 132)]
    #[case(SMALL_SAMPLE2.parse().unwrap(), 513)]
    fn checksum(#[case] input: DiskMap, #[case] expected: usize) {
        assert_eq!(expected, input.get_check_sum().unwrap());
    }
}
