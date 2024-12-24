use std::ops::{Index, IndexMut};

use iterators::{RowIterator, RowIteratorMut};

#[derive(Debug, PartialEq)]
pub enum Error {
    IncompatibleMatrixSize,
}

#[derive(Debug, PartialEq)]
pub struct Matrix<T> {
    rows: usize,
    columns: usize,
    data: Box<[T]>,
}

impl<T: Copy> Matrix<T> {
    pub fn init(initial_value: T, rows: usize, columns: usize) -> Self {
        let size = rows * columns;
        let data = vec![initial_value; size].into_boxed_slice();
        Self {
            rows,
            columns,
            data,
        }
    }
}

impl<T> Matrix<T> {
    pub fn from_vec(vector: Vec<T>, rows: usize, columns: usize) -> Result<Self, Error> {
        if vector.len() != rows * columns {
            Err(Error::IncompatibleMatrixSize)
        } else {
            Ok(Self {
                rows,
                columns,
                data: vector.into_boxed_slice(),
            })
        }
    }

    pub fn rows<'a>(&'a self) -> RowIterator<'a, T> {
        RowIterator::new(self)
    }

    pub fn rows_mut<'a>(&'a mut self) -> RowIteratorMut<'a, T> {
        RowIteratorMut::new(self)
    }

    pub fn row_count(&self) -> usize {
        self.rows
    }

    pub fn col_count(&self) -> usize {
        self.columns
    }

    pub fn transposed(&self) -> Matrix<&T> {
        let original_columns = self.columns;
        let original_rows = self.rows;
        let mut borrow_vector: Vec<&T> = Vec::new();
        if original_columns != 0 && original_rows != 0 {
            if original_columns == 1 || original_rows == 1 {
                borrow_vector = self.data.iter().collect();
            } else {
                for column in 0..original_columns {
                    for row in 0..original_rows {
                        borrow_vector.push(&self[(row, column)]);
                    }
                }
            }
        }
        Matrix::<&T> {
            rows: original_columns,
            columns: original_rows,
            data: borrow_vector.into_boxed_slice(),
        }
    }

    pub fn transposed_mut(&mut self) -> Matrix<&mut T> {
        let original_columns = self.columns;
        let original_rows = self.rows;
        let borrowed_refs: Vec<&mut T>;
        if original_columns == 0 || original_rows == 0 {
            borrowed_refs = Vec::new();
        } else if original_rows == 1 || original_columns == 1 {
            borrowed_refs = self.data.iter_mut().collect();
        } else {
            let mut borrow_vector: Vec<*mut T> = Vec::new();
            for column in 0..original_columns {
                for row in 0..original_rows {
                    borrow_vector.push(&mut self[(row, column)] as *mut T);
                }
            }
            unsafe {
                borrowed_refs = borrow_vector.into_iter().map(|ptr| &mut *ptr).collect();
            }
        }
        Matrix::<&mut T> {
            rows: original_columns,
            columns: original_rows,
            data: borrowed_refs.into_boxed_slice(),
        }
    }

    pub fn into_transposed(self) -> Self {
        let original_colums = self.columns;
        let original_rows = self.rows;
        let vector: Vec<T>;
        if original_rows == 0 || original_colums == 0 {
            vector = Vec::new();
        } else if original_rows == 1 || original_colums == 1 {
            vector = self.data.into_vec();
        } else {
            let mut temp: Vec<(usize, T)> = self
                .data
                .into_vec()
                .into_iter()
                .enumerate()
                .map(|(index, value)| {
                    let new_col = index / original_colums;
                    let new_row = index % original_colums;
                    let new_index = new_row * original_rows + new_col;
                    (new_index, value)
                })
                .collect();
            temp.sort_by_key(|(index, _)| *index);
            vector = temp.into_iter().map(|(_, val)| val).collect();
        }
        Self {
            rows: original_colums,
            columns: original_rows,
            data: vector.into_boxed_slice(),
        }
    }
}

impl<T: Default + Copy> Matrix<T> {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self::init(T::default(), rows, columns)
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, column) = index;
        if row >= self.rows || column >= self.columns {
            panic!("tried to index matrix out of bounds");
        } else {
            &self.data[row * self.columns + column]
        }
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, column) = index;
        if row >= self.rows || column >= self.columns {
            panic!("tried to index matrix out of bounds");
        } else {
            &mut self.data[row * self.columns + column]
        }
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.rows {
            panic!("tried to index matrix row out of bounds")
        } else {
            &self.data[index * self.columns..(index + 1) * self.columns]
        }
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.rows {
            panic!("tried to index matrix row out of bounds")
        } else {
            &mut self.data[index * self.columns..(index + 1) * self.columns]
        }
    }
}

mod iterators {
    use super::Matrix;

    pub struct RowIterator<'a, T> {
        row: usize,
        matrix: &'a Matrix<T>,
    }

    impl<'a, T> RowIterator<'a, T> {
        pub(super) fn new(arg: &'a Matrix<T>) -> Self {
            Self {
                row: 0usize,
                matrix: arg,
            }
        }
    }

    impl<'a, T> Iterator for RowIterator<'a, T> {
        type Item = &'a [T];
        fn next(&mut self) -> Option<Self::Item> {
            if self.row < self.matrix.rows {
                let row = &self.matrix[self.row];
                self.row += 1;
                Some(row)
            } else {
                None
            }
        }
    }

    pub struct RowIteratorMut<'a, T> {
        row: usize,
        matrix: &'a mut Matrix<T>,
    }

    impl<'a, T> RowIteratorMut<'a, T> {
        pub(super) fn new(arg: &'a mut Matrix<T>) -> Self {
            Self {
                row: 0usize,
                matrix: arg,
            }
        }
    }

    impl<'a, T> Iterator for RowIteratorMut<'a, T> {
        type Item = &'a mut [T];
        fn next(&mut self) -> Option<Self::Item> {
            if self.row < self.matrix.rows {
                let row = &mut self.matrix[self.row] as *mut [T];
                self.row += 1;
                unsafe { Some(&mut *row) }
            } else {
                None
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Matrix;
        use super::RowIterator;
        use super::RowIteratorMut;

        #[test]
        fn iter() {
            let matrix = Matrix::<i32>::from_vec(vec![-1, 0, 1, 0, 2, 1], 2, 3).unwrap();
            let mut iter = RowIterator {
                row: 0,
                matrix: &matrix,
            };
            assert_eq!(iter.next(), Some(&[-1i32, 0i32, 1i32][..]));
            assert_eq!(iter.next(), Some(&[0i32, 2i32, 1i32][..]));
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn iter_mut() {
            let mut matrix = Matrix::<i32>::init(1i32, 3, 3);
            let mut iter = RowIteratorMut {
                row: 0,
                matrix: &mut matrix,
            };

            assert_eq!(iter.next(), Some(&mut [1, 1, 1][..]));
            assert_eq!(iter.next(), Some(&mut [1, 1, 1][..]));
            assert_eq!(iter.next(), Some(&mut [1, 1, 1][..]));
            assert_eq!(iter.next(), None);

            let iter = RowIteratorMut {
                row: 0,
                matrix: &mut matrix,
            };

            for (i, row) in iter.enumerate() {
                for (j, val) in row.iter_mut().enumerate() {
                    *val = (i as i32) * 10 + j as i32;
                }
            }

            assert_eq!(
                matrix,
                Matrix::from_vec(vec![0, 1, 2, 10, 11, 12, 20, 21, 22], 3, 3).unwrap()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Error;
    use super::Matrix;

    #[test]
    fn new() {
        assert_eq!(
            Matrix::<u8>::new(5, 2),
            Matrix::<u8> {
                rows: 5,
                columns: 2,
                data: vec![0u8; 10].into_boxed_slice()
            }
        );
        assert_eq!(
            Matrix::<u32>::new(18, 356),
            Matrix::<u32> {
                rows: 18,
                columns: 356,
                data: vec![0u32; 6408].into_boxed_slice()
            }
        );
    }

    #[test]
    fn from_vec() {
        let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(
            Matrix::<i32>::from_vec(vector.clone(), 3usize, 3usize),
            Ok(Matrix::<i32> {
                rows: 3,
                columns: 3,
                data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_boxed_slice()
            })
        );
        assert_eq!(
            Matrix::<i32>::from_vec(vector.clone(), 2, 4),
            Err(Error::IncompatibleMatrixSize)
        );
        assert_eq!(
            Matrix::<i32>::from_vec(vector.clone(), 2, 5),
            Err(Error::IncompatibleMatrixSize)
        );
        assert_eq!(
            Matrix::<i32>::from_vec(vector.clone(), 1usize, 9usize),
            Ok(Matrix::<i32> {
                rows: 1,
                columns: 9,
                data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_boxed_slice()
            })
        );
        assert_eq!(
            Matrix::<i32>::from_vec(vector.clone(), 9usize, 1usize),
            Ok(Matrix::<i32> {
                rows: 9,
                columns: 1,
                data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_boxed_slice()
            })
        );
    }

    #[test]
    fn init() {
        assert_eq!(
            Matrix::<i16>::init(12i16, 2, 2),
            Matrix::<i16> {
                rows: 2,
                columns: 2,
                data: vec![12i16; 4].into_boxed_slice()
            }
        );
    }

    #[test]
    fn matrix_size() {
        let m = Matrix::<u8>::new(14, 9);
        assert_eq!(m.row_count(), 14);
        assert_eq!(m.col_count(), 9);
    }

    #[test]
    fn index() {
        let matrix = Matrix::<i16>::from_vec(vec![2, 3, 4, 5, 1, 7, 9, 0], 4, 2).unwrap();
        assert_eq!(matrix[(0, 0)], 2);
        assert_eq!(matrix[(0, 1)], 3);
        assert_eq!(matrix[(1, 0)], 4);
        assert_eq!(matrix[(1, 1)], 5);
        assert_eq!(matrix[(2, 0)], 1);
        assert_eq!(matrix[(2, 1)], 7);
        assert_eq!(matrix[(3, 0)], 9);
        assert_eq!(matrix[(3, 1)], 0);
        assert_eq!(matrix[0], [2, 3][..]);
        assert_eq!(matrix[1], [4, 5][..]);
        assert_eq!(matrix[2], [1, 7][..]);
        assert_eq!(matrix[3], [9, 0][..]);
    }

    #[test]
    #[should_panic]
    fn index_row_too_large() {
        let _ = Matrix::<isize>::new(2, 2)[(2, 0)];
    }

    #[test]
    #[should_panic]
    fn index_column_too_large() {
        let _ = Matrix::<isize>::new(2, 2)[(0, 2)];
    }

    #[test]
    fn mut_index() {
        let mut matrix = Matrix::<u128>::from_vec(vec![90, 3222, 77, 0], 2, 2).unwrap();
        matrix[(0, 1)] = 42u128;
        assert_eq!(
            matrix,
            Matrix::<u128> {
                rows: 2,
                columns: 2,
                data: vec![90, 42, 77, 0].into_boxed_slice()
            }
        );
    }

    #[test]
    #[should_panic]
    fn mut_index_row_too_large() {
        let mut matrix = Matrix::<i8>::new(2, 2);
        matrix[(2, 0)] = 1;
    }

    #[test]
    #[should_panic]
    fn mut_index_column_too_large() {
        let mut matrix = Matrix::<i8>::new(2, 2);
        matrix[(0, 2)] = 1;
    }

    #[test]
    fn transposed() {
        let m = Matrix::<u32>::from_vec(vec![1, 2, 3, 4, 5, 6], 2, 3).unwrap();
        assert_eq!(
            m.transposed(),
            Matrix::<&u32> {
                rows: 3,
                columns: 2,
                data: vec![&1, &4, &2, &5, &3, &6].into_boxed_slice()
            }
        );
    }

    #[test]
    fn transposed_mut() {
        let mut m = Matrix::<u32>::from_vec(vec![1, 2, 3, 4, 5, 6], 2, 3).unwrap();
        let mut transposed = m.transposed_mut();
        assert_eq!(
            transposed,
            Matrix::<&mut u32> {
                rows: 3,
                columns: 2,
                data: vec![&mut 1, &mut 4, &mut 2, &mut 5, &mut 3, &mut 6].into_boxed_slice()
            }
        );

        for (i, row) in transposed.rows_mut().enumerate() {
            for value in row.iter_mut() {
                **value += i as u32;
            }
        }
        assert_eq!(
            transposed,
            Matrix::<&mut u32> {
                rows: 3,
                columns: 2,
                data: vec![&mut 1, &mut 4, &mut 3, &mut 6, &mut 5, &mut 8].into_boxed_slice()
            }
        );
        assert_eq!(
            m,
            Matrix::<u32> {
                rows: 2,
                columns: 3,
                data: vec![1, 3, 5, 4, 6, 8].into_boxed_slice()
            }
        );
    }

    #[test]
    fn into_transposed() {
        let m = Matrix::<usize>::from_vec(vec![1, 2, 3, 4, 5, 6], 2, 3).unwrap();
        assert_eq!(
            m.into_transposed(),
            Matrix::<usize> {
                rows: 3,
                columns: 2,
                data: vec![1, 4, 2, 5, 3, 6].into_boxed_slice()
            }
        );
    }
}