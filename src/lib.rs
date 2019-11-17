use std::error::Error as StdError;
use std::fmt;

/// Enum representing the quality of an answer from 0 to 5.
/// - 0 - complete blackout.
/// - 1 - incorrect response; the correct one remembered
/// - 2 - incorrect response; where the correct one seemed easy to recall
/// - 3 - correct response recalled with serious difficulty
/// - 4 - correct response after a hesitation
/// - 5 - perfect response
#[derive(Debug)]
pub struct Quality {
    value: u8,
}

#[derive(Debug)]
pub enum Error {
    /// The maximum value for the quality of an answer is 5.
    /// This error is for when an answer above 5 is given.
    QualityAboveFiveError(u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::QualityAboveFiveError(q) => {
                write!(f, "Quality cannot be greater than 5, {} was given", q)
            }
        }
    }
}

impl StdError for Error {}

impl Quality {
    /// Create a `Quality` object from a number between 0 and 5.
    pub fn from(q: u8) -> Result<Self, Error> {
        match q {
            0 | 1 | 2 | 3 | 4 | 5 => Ok(Self { value: q }),
            _ => Err(Error::QualityAboveFiveError(q)),
        }
    }
}

/// A struct that holds the essential metadata for an item using the supermemo2 algorithm.
#[derive(Debug)]
pub struct Item {
    /// The number of reviews of this item.
    repetitions: usize,
    /// Easiness factor.
    efactor: f64,
}

impl Item {
    /// Return an `Item` with no reviews and the default E-factor of 2.5.
    pub fn new() -> Self {
        Self {
            repetitions: 0,
            efactor: 2.5,
        }
    }

    /// Create an `Item` that has already been reviewed by specifying the
    /// number of repetitions done and the current E-factor.
    pub fn from(repetitions: usize, efactor: f64) -> Self {
        Self {
            repetitions,
            efactor,
        }
    }

    /// Returns the current interval of the item.
    /// The interval is defined as the time in days since the previous review after which
    /// this item will be due for review.
    pub fn interval(&self) -> usize {
        match self.repetitions {
            0 => 0,
            1 => 1,
            2 => 6,
            _ => (6.0 * self.efactor.powi(self.repetitions as i32 - 2)).ceil() as usize,
        }
    }

    fn new_efactor(mut efactor: f64, quality: &Quality) -> f64 {
        // EF':=EF+(0.1-(5-q)*(0.08+(5-q)*0.02))
        if efactor < 1.3 {
            efactor = 1.3;
        }

        efactor
            + (0.1 - (5.0 - quality.value as f64) * (0.08 + (5.0 - quality.value as f64) * 0.02))
    }

    fn new_repetitions(repetitions: usize, quality: &Quality) -> usize {
        match quality.value {
            0 | 1 | 2 => 1,
            _ => repetitions + 1,
        }
    }

    /// Returns a new `Item` for an answer to this `Item` with the provided `Quality`.
    pub fn answer(&self, quality: &Quality) -> Self {
        Self {
            repetitions: Self::new_repetitions(self.repetitions, quality),
            efactor: Self::new_efactor(self.efactor, quality),
        }
    }

    /// Updates an `Item` for an answer with the provided `Quality`.
    pub fn answer_mut(&mut self, quality: &Quality) {
        self.repetitions = Self::new_repetitions(self.repetitions, quality);
        self.efactor = Self::new_efactor(self.efactor, quality);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn quality_above_5() {
        Quality::from(6).unwrap();
    }

    #[test]
    fn answer() {
        let q = Quality::from(5).unwrap();
        let item = Item::from(3, 2.4);
        let answer = item.answer(&q);
        assert_eq!(answer.repetitions, 4);
        assert_eq!(answer.efactor, 2.5);
    }

    #[test]
    fn interval() {
        let item = Item::from(5, 3.9);
        assert_eq!(item.interval(), 356);
    }
}
