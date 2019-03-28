#[derive(Debug, PartialEq)]
pub struct NonZeroPosInteger {
    n: u32,
}

impl NonZeroPosInteger {
    pub fn new(n: u32) -> NonZeroPosInteger {
        if n == 0 {
            panic!("Input must be an integer greater than zero.")
        }

        NonZeroPosInteger { n }
    }

    pub fn value(&self) -> u32 {
        self.n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_with_valid_input() {
        let expected = NonZeroPosInteger { n: 5 };
        assert_eq!(expected, NonZeroPosInteger::new(5));
    }

    #[test]
    #[should_panic(expected = "Input must be an integer greater than zero.")]
    fn new_with_invalid_input() {
        NonZeroPosInteger::new(0);
    }

    #[test]
    fn get_value() {
        assert_eq!(2424, NonZeroPosInteger::new(2424).value());
    }
}
