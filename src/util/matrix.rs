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
    use rstest::rstest;
    use std::fmt::Debug;
    use std::ops::AddAssign;
    use std::usize;

    use super::Error;
    use super::Matrix;

    #[rstest]
    #[case(5, 2, Matrix::<u8> { rows: 5, columns: 2, data: vec![0u8;10].into_boxed_slice()})]
    #[case(18, 356, Matrix::<u32> { rows: 18, columns: 356, data: vec![0u32;6408].into_boxed_slice()})]
    #[case(0, 356, Matrix::<i32> { rows: 0, columns: 356, data: vec![].into_boxed_slice()})]
    #[case(9, 0, Matrix::<isize> { rows: 9, columns: 0, data: vec![].into_boxed_slice()})]
    fn new<T: Copy + Default + Debug + PartialEq>(
        #[case] row_count: usize,
        #[case] column_count: usize,
        #[case] expected: Matrix<T>,
    ) {
        assert_eq!(expected, Matrix::<T>::new(row_count, column_count));
    }

    #[rstest]
    #[case(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3, Ok(Matrix::<i32> { rows: 3, columns: 3, data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_boxed_slice() }))]
    #[case(vec![1, 2 ], 1, 3, Err(Error::IncompatibleMatrixSize))]
    #[case(vec![1, 2 ], 1, 1, Err(Error::IncompatibleMatrixSize))]
    #[case(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 1, 9, Ok(Matrix::<u16> { rows: 1, columns: 9, data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_boxed_slice() }))]
    #[case(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 9, 1, Ok(Matrix::<u16> { rows: 9, columns: 1, data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_boxed_slice() }))]
    fn from_vec<T: Debug + PartialEq>(
        #[case] vector: Vec<T>,
        #[case] row_count: usize,
        #[case] column_count: usize,
        #[case] expected: Result<Matrix<T>, Error>,
    ) {
        assert_eq!(
            expected,
            Matrix::<T>::from_vec(vector, row_count, column_count)
        );
    }

    #[rstest]
    #[case(12i16, 2, 2, Matrix::<i16> { rows: 2, columns: 2, data: vec![12i16; 4].into_boxed_slice() })]
    #[case(9i128, 100, 39, Matrix::<i128> { rows: 100, columns: 39, data: vec![9i128; 3900].into_boxed_slice() })]
    #[case(-1i8, 2, 4000, Matrix::<i8> { rows: 2, columns: 4000, data: vec![-1i8; 8000].into_boxed_slice() })]
    fn init<T: Copy + Debug + PartialEq>(
        #[case] initial_value: T,
        #[case] row_count: usize,
        #[case] column_count: usize,
        #[case] expected: Matrix<T>,
    ) {
        assert_eq!(
            expected,
            Matrix::<T>::init(initial_value, row_count, column_count)
        );
    }

    #[rstest]
    #[case(Matrix::<i16>::new(18, 29), 18, 29)]
    #[case(Matrix::<i64>::new(99,  4), 99,  4)]
    #[case(Matrix::<i32>::new( 8,  5),  8,  5)]
    fn matrix_size<T>(
        #[case] m: Matrix<T>,
        #[case] expected_row_count: usize,
        #[case] expected_column_count: usize,
    ) {
        assert_eq!(expected_row_count, m.row_count());
        assert_eq!(expected_column_count, m.col_count());
    }

    enum IndexType {
        Cell((usize, usize)),
        Row(usize),
    }

    enum IndexResultType<'a, T> {
        Cell(&'a T),
        Row(&'a [T]),
    }

    #[rstest]
    #[case(IndexType::Cell((0, 0)), IndexResultType::Cell(&2))]
    #[case(IndexType::Cell((0, 1)), IndexResultType::Cell(&3))]
    #[should_panic(expected = "out of bounds")]
    #[case(IndexType::Cell((0, 2)), IndexResultType::Cell(&0))]
    #[should_panic(expected = "out of bounds")]
    #[case(IndexType::Cell((0, 9)), IndexResultType::Cell(&0))]
    #[case(IndexType::Cell((1, 0)), IndexResultType::Cell(&4))]
    #[case(IndexType::Cell((1, 1)), IndexResultType::Cell(&5))]
    #[case(IndexType::Cell((2, 0)), IndexResultType::Cell(&1))]
    #[case(IndexType::Cell((2, 1)), IndexResultType::Cell(&7))]
    #[case(IndexType::Cell((3, 0)), IndexResultType::Cell(&9))]
    #[case(IndexType::Cell((3, 1)), IndexResultType::Cell(&0))]
    #[should_panic(expected = "out of bounds")]
    #[case(IndexType::Cell((4, 0)), IndexResultType::Cell(&0))]
    #[should_panic(expected = "out of bounds")]
    #[case(IndexType::Cell((9, 0)), IndexResultType::Cell(&0))]
    #[should_panic(expected = "out of bounds")]
    #[case(IndexType::Cell((4, 9)), IndexResultType::Cell(&0))]
    #[should_panic(expected = "out of bounds")]
    #[case(IndexType::Cell((9, 9)), IndexResultType::Cell(&0))]
    #[case(IndexType::Row(0),       IndexResultType::Row(&[2, 3]))]
    #[case(IndexType::Row(1),       IndexResultType::Row(&[4, 5]))]
    #[case(IndexType::Row(2),       IndexResultType::Row(&[1, 7]))]
    #[case(IndexType::Row(3),       IndexResultType::Row(&[9, 0]))]
    #[should_panic(expected = "out of bounds")]
    #[case(IndexType::Row(4),       IndexResultType::Row(&[]))]
    #[should_panic(expected = "out of bounds")]
    #[case(IndexType::Row(9),       IndexResultType::Row(&[]))]
    fn indexing(#[case] index: IndexType, #[case] expected: IndexResultType<usize>) {
        let matrix = Matrix::<usize>::from_vec(vec![2, 3, 4, 5, 1, 7, 9, 0], 4, 2).unwrap();
        match (index, expected) {
            (IndexType::Cell(index), IndexResultType::Cell(expected)) => {
                assert_eq!(expected, &matrix[index])
            }
            (IndexType::Row(index), IndexResultType::Row(expected)) => {
                assert_eq!(expected, &matrix[index])
            }
            _ => panic!("missmatched index types"),
        }
    }

    #[rstest]
    #[case(IndexType::Cell((0, 0)), IndexResultType::Cell(&12), Matrix::<u64> { rows: 2, columns: 2, data: vec![12, 3222, 77, 0].into_boxed_slice()})]
    #[case(IndexType::Cell((0, 1)), IndexResultType::Cell(&80), Matrix::<u64> { rows: 2, columns: 2, data: vec![90, 80, 77, 0].into_boxed_slice()})]
    #[should_panic(expected = "out of bounds")]
    #[case(IndexType::Cell((0, 2)), IndexResultType::Cell(&0), Matrix::<u64> { rows: 0, columns: 0, data: vec![].into_boxed_slice()})]
    #[should_panic(expected = "out of bounds")]
    #[case(IndexType::Cell((0, 9)), IndexResultType::Cell(&0), Matrix::<u64> { rows: 0, columns: 0, data: vec![].into_boxed_slice()})]
    #[case(IndexType::Cell((1, 0)), IndexResultType::Cell(&4), Matrix::<u64> { rows: 2, columns: 2, data: vec![90, 3222, 4, 0].into_boxed_slice()})]
    #[case(IndexType::Cell((1, 1)), IndexResultType::Cell(&13), Matrix::<u64> { rows: 2, columns: 2, data: vec![90, 3222, 77, 13].into_boxed_slice()})]
    #[should_panic(expected = "out of bounds")]
    #[case(IndexType::Cell((2, 0)), IndexResultType::Cell(&0), Matrix::<u64> { rows: 0, columns: 0, data: vec![].into_boxed_slice()})]
    #[should_panic(expected = "out of bounds")]
    #[case(IndexType::Cell((9, 0)), IndexResultType::Cell(&0), Matrix::<u64> { rows: 0, columns: 0, data: vec![].into_boxed_slice()})]
    #[should_panic(expected = "out of bounds")]
    #[case(IndexType::Cell((2, 2)), IndexResultType::Cell(&0), Matrix::<u64> { rows: 0, columns: 0, data: vec![].into_boxed_slice()})]
    #[should_panic(expected = "out of bounds")]
    #[case(IndexType::Cell((9, 9)), IndexResultType::Cell(&0), Matrix::<u64> { rows: 0, columns: 0, data: vec![].into_boxed_slice()})]
    #[case(IndexType::Row(0), IndexResultType::Row(&[1, 2]), Matrix::<u64> { rows: 2, columns: 2, data: vec![1, 2, 77, 0].into_boxed_slice()})]
    #[case(IndexType::Row(1), IndexResultType::Row(&[89, 5]), Matrix::<u64> { rows: 2, columns: 2, data: vec![90, 3222, 89, 5].into_boxed_slice()})]
    #[should_panic(expected = "out of bounds")]
    #[case(IndexType::Row(2), IndexResultType::Row(&[]), Matrix::<u64> { rows: 0, columns: 0, data: vec![].into_boxed_slice()})]
    #[should_panic(expected = "out of bounds")]
    #[case(IndexType::Row(9), IndexResultType::Row(&[]), Matrix::<u64> { rows: 0, columns: 0, data: vec![].into_boxed_slice()})]
    fn mut_indexing(
        #[case] index: IndexType,
        #[case] value: IndexResultType<u64>,
        #[case] expected: Matrix<u64>,
    ) {
        let mut matrix = Matrix::<u64>::from_vec(vec![90, 3222, 77, 0], 2, 2).unwrap();
        match (index, value) {
            (IndexType::Cell(index), IndexResultType::Cell(value)) => matrix[index] = *value,
            (IndexType::Row(index), IndexResultType::Row(value)) => {
                for (x, y) in matrix[index].iter_mut().zip(value.iter()) {
                    *x = *y
                }
            }
            _ => panic!("missmatched index types"),
        }
        assert_eq!(expected, matrix);
    }

    #[rstest]
    #[case(Matrix::<u128> { rows: 2, columns: 3, data: vec![1, 2, 3, 4, 5, 6].into_boxed_slice() }, Matrix::<&u128> { rows: 3, columns: 2, data: vec![&1, &4, &2, &5, &3, &6].into_boxed_slice() })]
    #[case(Matrix::<i128> { rows: 3, columns: 2, data: vec![1, 2, 3, 4, 5, 6].into_boxed_slice() }, Matrix::<&i128> { rows: 2, columns: 3, data: vec![&1, &3, &5, &2, &4, &6].into_boxed_slice() })]
    #[case(Matrix::<u16> { rows: 2, columns: 2, data: vec![1, 2, 3, 4].into_boxed_slice() }, Matrix::<&u16> { rows: 2, columns: 2, data: vec![&1, &3, &2, &4].into_boxed_slice() })]
    #[case(Matrix::<i16> { rows: 1, columns: 5, data: vec![1, 2, 3, 4, 5].into_boxed_slice() }, Matrix::<&i16> { rows: 5, columns: 1, data: vec![&1, &2, &3, &4, &5].into_boxed_slice() })]
    #[case(Matrix::<u32> { rows: 5, columns: 1, data: vec![1, 2, 3, 4, 5].into_boxed_slice() }, Matrix::<&u32> { rows: 1, columns: 5, data: vec![&1, &2, &3, &4, &5].into_boxed_slice() })]
    #[case(Matrix::<u8> { rows: 0, columns: 2, data: vec![].into_boxed_slice() }, Matrix::<&u8> { rows: 2, columns: 0, data: vec![].into_boxed_slice() })]
    #[case(Matrix::<i8> { rows: 2, columns: 0, data: vec![].into_boxed_slice() }, Matrix::<&i8> { rows: 0, columns: 2, data: vec![].into_boxed_slice() })]
    #[case(Matrix::<u32> { rows: 0, columns: 0, data: vec![].into_boxed_slice() }, Matrix::<&u32> { rows: 0, columns: 0, data: vec![].into_boxed_slice() })]
    fn transposed<T: Debug + PartialEq>(#[case] m: Matrix<T>, #[case] expected: Matrix<&T>) {
        assert_eq!(expected, m.transposed());
    }

    #[rstest]
    #[case(Matrix::<i32> { rows: 2, columns: 3, data: vec![1, 2, 3, 4, 5, 6].into_boxed_slice() }, Matrix::<i32> { rows: 2, columns: 3, data: vec![1, 3, 5, 4, 6, 8].into_boxed_slice() })]
    #[case(Matrix::<u32> { rows: 3, columns: 2, data: vec![1, 2, 3, 4, 5, 6].into_boxed_slice() }, Matrix::<u32> { rows: 3, columns: 2, data: vec![1, 3, 3, 5, 5, 7].into_boxed_slice() })]
    #[case(Matrix::<u16> { rows: 2, columns: 2, data: vec![1, 2, 3, 4].into_boxed_slice() }, Matrix::<u16> { rows: 2, columns: 2, data: vec![1, 3, 3, 5].into_boxed_slice() })]
    #[case(Matrix::<i16> { rows: 1, columns: 5, data: vec![1, 2, 3, 4, 5].into_boxed_slice() }, Matrix::<i16> { rows: 1, columns: 5, data: vec![1, 3, 5, 7, 9].into_boxed_slice() })]
    #[case(Matrix::<i64> { rows: 5, columns: 1, data: vec![1, 2, 3, 4, 5].into_boxed_slice() }, Matrix::<i64> { rows: 5, columns: 1, data: vec![1, 2, 3, 4, 5].into_boxed_slice() })]
    #[case(Matrix::<u8> { rows: 0, columns: 2, data: vec![].into_boxed_slice() }, Matrix::<u8> { rows: 0, columns: 2, data: vec![].into_boxed_slice() })]
    #[case(Matrix::<i8> { rows: 2, columns: 0, data: vec![].into_boxed_slice() }, Matrix::<i8> { rows: 2, columns: 0, data: vec![].into_boxed_slice() })]
    #[case(Matrix::<u32> { rows: 0, columns: 0, data: vec![].into_boxed_slice() }, Matrix::<u32> { rows: 0, columns: 0, data: vec![].into_boxed_slice() })]
    fn transposed_mut<T: AddAssign + Debug + PartialEq + TryFrom<usize>>(
        #[case] m: Matrix<T>,
        #[case] expected: Matrix<T>,
    ) where
        <T as TryFrom<usize>>::Error: Debug,
    {
        let mut m = m;
        for (i, row) in m.transposed_mut().rows_mut().enumerate() {
            for value in row.iter_mut() {
                **value += T::try_from(i).unwrap()
            }
        }
        assert_eq!(expected, m);
    }

    #[rstest]
    #[case(Matrix::<u128> { rows: 2, columns: 3, data: vec![1, 2, 3, 4, 5, 6].into_boxed_slice() }, Matrix::<u128> { rows: 3, columns: 2, data: vec![1, 4, 2, 5, 3, 6].into_boxed_slice() })]
    #[case(Matrix::<i128> { rows: 3, columns: 2, data: vec![1, 2, 3, 4, 5, 6].into_boxed_slice() }, Matrix::<i128> { rows: 2, columns: 3, data: vec![1, 3, 5, 2, 4, 6].into_boxed_slice() })]
    #[case(Matrix::<u16> { rows: 2, columns: 2, data: vec![1, 2, 3, 4].into_boxed_slice() }, Matrix::<u16> { rows: 2, columns: 2, data: vec![1, 3, 2, 4].into_boxed_slice() })]
    #[case(Matrix::<i16> { rows: 1, columns: 5, data: vec![1, 2, 3, 4, 5].into_boxed_slice() }, Matrix::<i16> { rows: 5, columns: 1, data: vec![1, 2, 3, 4, 5].into_boxed_slice() })]
    #[case(Matrix::<u32> { rows: 5, columns: 1, data: vec![1, 2, 3, 4, 5].into_boxed_slice() }, Matrix::<u32> { rows: 1, columns: 5, data: vec![1, 2, 3, 4, 5].into_boxed_slice() })]
    #[case(Matrix::<u8> { rows: 0, columns: 2, data: vec![].into_boxed_slice() }, Matrix::<u8> { rows: 2, columns: 0, data: vec![].into_boxed_slice() })]
    #[case(Matrix::<i8> { rows: 2, columns: 0, data: vec![].into_boxed_slice() }, Matrix::<i8> { rows: 0, columns: 2, data: vec![].into_boxed_slice() })]
    #[case(Matrix::<u32> { rows: 0, columns: 0, data: vec![].into_boxed_slice() }, Matrix::<u32> { rows: 0, columns: 0, data: vec![].into_boxed_slice() })]
    fn into_transposed<T: Debug + PartialEq>(#[case] m: Matrix<T>, #[case] expected: Matrix<T>) {
        assert_eq!(expected, m.into_transposed())
    }
}
