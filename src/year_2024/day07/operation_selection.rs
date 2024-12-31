use super::error::Error;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Addition,
    Multiplication,
    Concatination,
}

pub fn get_bit_vector(unsigned_integer: usize, len: usize) -> Result<Vec<bool>, Error> {
    if unsigned_integer > (1 << len) - 1 {
        Err(Error::BitFieldGeneration)
    } else {
        let mut unsigned_integer = unsigned_integer;
        let mut bit_vector = Vec::new();
        while bit_vector.len() < len {
            bit_vector.push(unsigned_integer % 2 == 1);
            unsigned_integer >>= 1; // divide by 2
        }
        Ok(bit_vector)
    }
}

pub fn get_enumeration_vector(unsigned_integer: u32, len: usize) -> Result<Vec<Operation>, Error> {
    let interger_u32: u32 = unsigned_integer
        .try_into()
        .map_err(|_| Error::EnumerationFieldGeneration)?;
    let maximum_displayable_value: u32 = 3u32.pow(
        len.try_into()
            .map_err(|_| Error::EnumerationFieldGeneration)?,
    ) - 1u32;
    if interger_u32 > maximum_displayable_value {
        Err(Error::EnumerationFieldGeneration)
    } else {
        let mut unsigned_integer = unsigned_integer;
        let mut enumeration_vector = Vec::new();
        while enumeration_vector.len() < len {
            enumeration_vector.push(match unsigned_integer % 3 {
                0 => Ok(Operation::Multiplication),
                1 => Ok(Operation::Addition),
                2 => Ok(Operation::Concatination),
                _ => Err(Error::EnumerationFieldGeneration),
            }?);
            unsigned_integer /= 3;
        }
        Ok(enumeration_vector)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::get_bit_vector;
    use super::get_enumeration_vector;
    use super::Error;
    use super::Operation;

    #[rstest]
    #[case(1, 4, Ok(vec![true, false, false, false]))]
    #[case(5, 4, Ok(vec![true, false, true, false]))]
    #[case(5, 2, Err(Error::BitFieldGeneration))]
    fn generate_bit_vector(
        #[case] value_to_encode: usize,
        #[case] target_vector_size: usize,
        #[case] expected: Result<Vec<bool>, Error>,
    ) {
        assert_eq!(
            expected,
            get_bit_vector(value_to_encode, target_vector_size)
        )
    }

    #[rstest]
    #[case(5, 4, Ok(vec![ Operation::Concatination, Operation::Addition, Operation::Multiplication, Operation::Multiplication ]))]
    #[case(6, 4, Ok(vec![ Operation::Multiplication, Operation::Concatination, Operation::Multiplication, Operation::Multiplication ]))]
    #[case(27, 2, Err(Error::EnumerationFieldGeneration))]
    fn generate_enumeration_vector(
        #[case] value_to_encode: u32,
        #[case] target_vector_size: usize,
        #[case] expected: Result<Vec<Operation>, Error>,
    ) {
        assert_eq!(
            expected,
            get_enumeration_vector(value_to_encode, target_vector_size)
        )
    }
}
