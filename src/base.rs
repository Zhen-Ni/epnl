use std::ops::{Deref, DerefMut};

pub const TOBSIZE: usize = 24;
pub type TOBScalarType = f64;
pub type TOBArray = [TOBScalarType; TOBSIZE];


#[derive(Debug, Clone)]
pub struct TOBStorage {
    pub(crate) storage: TOBArray
}

impl TOBStorage {
    /// Constructs a new SPL object with nan values.
    ///
    /// # Examples
    /// ```
    /// use epnl::TOBStorage;
    /// let tob_data = TOBStorage::new();
    /// let tob_50 = tob_data.get(1);
    /// // All values in `tob_data` are initialized to nan.
    /// assert!(f64::is_nan(tob_50));
    /// ```
    pub fn new() -> TOBStorage {
        TOBStorage{ storage: [TOBScalarType::NAN; TOBSIZE]}
    }

    /// Get the i-th band in the TOB list.  To be consistent with ICAO
    /// ANNEX 16, the index starts from 1 and ends with 24 (included).
    ///
    /// ```
    /// use epnl::TOBFREQ;
    /// let freq = TOBFREQ.clone();
    /// let freq50 = freq.get(1);
    /// assert_eq!(freq50, 50.0);
    /// ```
    pub fn get(&self, index: usize) -> TOBScalarType {
        self[index - 1]
    }

    /// Set the i-th band in the TOB list to given value.  To be
    /// consistent with ICAO ANNEX 16, the index starts from 1 and
    /// ends with 24 (included).
    ///
    /// ```
    /// use epnl::TOBStorage;
    /// let mut tob_data = TOBStorage::new();
    /// tob_data.set(5, 75.0);
    /// let data_125 = tob_data.get(5);
    /// assert_eq!(data_125, 75.0);
    /// ```
    pub fn set(&mut self, index: usize, value: TOBScalarType) {
        self[index - 1] = value;
    }
}


impl Deref for TOBStorage {
    type Target = TOBArray;
    
    fn deref(&self) -> &Self::Target {
        &self.storage
    }
}

impl DerefMut for TOBStorage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.storage    
    }
}
    

impl From<TOBArray> for TOBStorage {
    /// Converts an array into a TOB object.
    fn from(value: TOBArray) -> TOBStorage {
        TOBStorage::from(&value[..])
        // TOBStorage {storage: value.try_into().expect(&format!(
        //     "Expect a Vec of length {}, got length {} instead.",
        //     TOBSIZE, value.len()))}
    }
}


impl From<&[TOBScalarType]> for TOBStorage {
    /// Converts an array into a TOB object.
    fn from(value: &[TOBScalarType]) -> TOBStorage {
        TOBStorage {storage: value.try_into().expect(&format!(
            "Expect a Vec of length {}, got length {} instead.",
            TOBSIZE, value.len()))}
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage() {
        let mut storage = TOBStorage::from([0.0; 24]);
        storage.set(5, 75.0);
        let spl125 = storage.get(5);
        assert_eq!(spl125, 75.0);
        assert_eq!(storage[4], 75.0);
     }
}
