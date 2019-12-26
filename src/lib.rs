/*!
# supermemo2

This crate implements the core components of the supermemo2 spaced repetition algorithm.

# Examples

```rust
use supermemo2::Item;

pub fn main() {
    let item = Item::default();
    let interval = item
        .review(4)
        .unwrap()
        .review(3)
        .unwrap()
        .review(5)
        .unwrap()
        .interval();

    assert_eq!(interval, 15);
}
```
*/

use std::default::Default;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Error {
    /// The maximum value for the quality of an answer is 5.
    /// This error is for when an answer above 5 is given.
    QualityAboveFiveError(u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::QualityAboveFiveError(q) => {
                write!(f, "Quality cannot be greater than 5, {} was given.", q)
            }
        }
    }
}

impl StdError for Error {}

/// A struct that holds the essential metadata for an item using the supermemo2 algorithm.
#[derive(Debug, Copy, Clone)]
pub struct Item {
    /// The number of reviews of this item.
    repetitions: usize,
    /// Easiness factor.
    efactor: f64,
}

impl Default for Item {
    /// Return a default new `Item` with 0 repetitions and an E-factor of 2.5.
    fn default() -> Self {
        Self {
            repetitions: 0,
            efactor: 2.5,
        }
    }
}

impl Item {
    /// Return an `Item` with the given number of repetitions and E-factor.
    pub fn new(repetitions: usize, efactor: f64) -> Self {
        Self {
            repetitions,
            efactor,
        }
    }

    /// Get the number of repetitions of this `Item`.
    pub fn repetitions(&self) -> usize {
        self.repetitions
    }

    /// Get the E-factor of this `Item`.
    pub fn efactor(&self) -> f64 {
        self.efactor
    }

    /// Returns the current interval of the `Item`.
    /// The interval is defined as the time in days since the previous review after which
    /// this `Item` will be due for review.
    pub fn interval(&self) -> usize {
        match self.repetitions {
            0 => 0,
            1 => 1,
            2 => 6,
            _ => (6.0 * self.efactor.powi(self.repetitions as i32 - 2)).ceil() as usize,
        }
    }

    fn new_efactor(&self, quality: u8) -> Result<f64, Error> {
        let ef = if self.efactor < 1.3 {
            1.3
        } else {
            self.efactor
        };

        if quality > 5 {
            Err(Error::QualityAboveFiveError(quality))
        } else {
            // EF':=EF+(0.1-(5-q)*(0.08+(5-q)*0.02))
            Ok(ef + (0.1 - (5.0 - quality as f64) * (0.08 + (5.0 - quality as f64) * 0.02)))
        }
    }

    fn new_repetitions(&self, quality: u8) -> Result<usize, Error> {
        match quality {
            0 | 1 | 2 => Ok(1),
            3 | 4 | 5 => Ok(self.repetitions + 1),
            _ => Err(Error::QualityAboveFiveError(quality)),
        }
    }

    /// Returns a new `Item` based on the given quality.
    /// The quality can be an integer between 0 and 5.
    /// If a quality above 5 is given, this will return an `Err`.
    /// - 0 - complete blackout.
    /// - 1 - incorrect response; the correct one remembered
    /// - 2 - incorrect response; where the correct one seemed easy to recall
    /// - 3 - correct response recalled with serious difficulty
    /// - 4 - correct response after a hesitation
    /// - 5 - perfect response
    pub fn review(&self, quality: u8) -> Result<Self, Error> {
        Ok(Self {
            repetitions: self.new_repetitions(quality)?,
            efactor: self.new_efactor(quality)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_default_values() {
        let item = Item::default();
        assert_eq!(item.repetitions, 0);
        assert_eq!(item.efactor, 2.5);
    }

    #[test]
    #[should_panic]
    fn quality_above_5_returns_error() {
        let item = Item::default();
        item.review(6).unwrap();
    }

    #[test]
    fn review_gives_correct_repetitions_and_efactor() {
        let item = Item::new(3, 2.4);
        let new_item = item.review(5).unwrap();
        assert_eq!(new_item.repetitions, 4);
        assert_eq!(new_item.efactor, 2.5);
    }

    #[test]
    fn calculate_interval() {
        let item = Item::new(5, 3.9);
        assert_eq!(item.interval(), 356);
    }
}
