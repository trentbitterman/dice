//! # Num
//!
//! Defines number types needed for the proper functioning
//! of `dice`.

/// # NonZeroPosInteger
///
/// A number type that holds integers greater than,
/// and not including, zero.
#[derive(Debug, PartialEq)]
pub struct NonZeroPosInteger {
    n: u32,
}

impl NonZeroPosInteger {

    /// Creates a new NonZeroPosInteger.
    ///
    /// # Examples
    ///
    /// ```
    /// use dice::num;
    ///
    /// let five = num::NonZeroPosInteger::new(5);
    ///
    /// let thirty = num::NonZeroPosInteger::new(30);
    /// ```
    ///
    /// # Panics
    ///
    /// Will panic if value given is 0.
    pub fn new(n: u32) -> NonZeroPosInteger {
        if n == 0 {
            panic!("Input must be an integer greater than zero.")
        }

        NonZeroPosInteger { n }
    }

    /// Returns the value of the NonZeroPosInteger.
    ///
    /// # Examples
    ///
    /// ```
    /// use dice::num;
    ///
    /// let seventy_six = num::NonZeroPosInteger::new(76);
    ///
    /// assert_eq!(76, seventy_six.value());
    /// ```
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
