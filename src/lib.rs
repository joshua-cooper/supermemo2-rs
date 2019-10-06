use std::cmp::Ordering;
use std::error::Error as StdError;
use std::fmt;

/// Enum representing the quality of an answer from 0 to 5.
/// 0 - complete blackout.
/// 1 - incorrect response; the correct one remembered
/// 2 - incorrect response; where the correct one seemed easy to recall
/// 3 - correct response recalled with serious difficulty
/// 4 - correct response after a hesitation
/// 5 - perfect response
pub enum Quality {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Ord for Quality {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = match self {
            Quality::Zero => 0,
            Quality::One => 1,
            Quality::Two => 2,
            Quality::Three => 3,
            Quality::Four => 4,
            Quality::Five => 5,
        };

        let b = match other {
            Quality::Zero => 0,
            Quality::One => 1,
            Quality::Two => 2,
            Quality::Three => 3,
            Quality::Four => 4,
            Quality::Five => 5,
        };

        a.cmp(&b)
    }
}

impl Eq for Quality {}

impl PartialOrd for Quality {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Quality {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl From<&Quality> for f64 {
    fn from(q: &Quality) -> Self {
        match q {
            Quality::Zero => 0.0,
            Quality::One => 1.0,
            Quality::Two => 2.0,
            Quality::Three => 3.0,
            Quality::Four => 4.0,
            Quality::Five => 5.0,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    /// The maximum value for the quality of an answer is 5.
    /// This error is for when an answer above 5 is given.
    QualityAboveFiveError(usize),
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
    pub fn from(q: usize) -> Result<Self, Error> {
        match q {
            0 => Ok(Self::Zero),
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Five),
            _ => Err(Error::QualityAboveFiveError(q)),
        }
    }
}

/// A struct that holds the essential metadata for an item using the supermemo2 algorithm.
pub struct Item {
    /// The number of reviews of this item.
    repetitions: usize,
    /// Easiness factor.
    efactor: f64,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Repetitions: {}, E-factor: {}",
            self.repetitions, self.efactor
        )
    }
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

    /// Returns the current interval of the card.
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
        let q = f64::from(quality);

        if efactor < 1.3 {
            efactor = 1.3;
        }

        efactor + (0.1 - (5.0 - q) * (0.08 + (5.0 - q) * 0.02))
    }

    fn new_repetitions(repetitions: usize, quality: &Quality) -> usize {
        if quality < &Quality::Three {
            return 1;
        } else {
            return repetitions + 1;
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
}
