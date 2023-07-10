
use std::ops::{Deref, DerefMut};

use crate::{TOBStorage, TOBArray, TOBScalarType};


#[derive(Debug, Clone)]
pub struct SPL {
    storage: TOBStorage
}

impl SPL {
    /// Constructs a new SPL object with nan values.
    ///
    /// # Examples
    /// ```
    /// use epnl::SPL;
    /// let spl = SPL::new();
    /// let spl50 = spl.get(1);
    /// // All values in `spl` are initialized to nan.
    /// assert!(f64::is_nan(spl50));
    /// ```
    pub fn new() -> SPL {
        SPL{ storage: TOBStorage::new()}
    }
}

impl From<TOBStorage> for SPL {
    /// Converts a TOB storage object into a TOB object.
    ///
    /// # Examples
    /// ```
    /// use epnl::{SPL, TOBStorage};
    /// let spl_values = TOBStorage::from([1.0; 24]);
    /// let spl = SPL::from(spl_values);
    /// ```    
    fn from(value: TOBStorage) -> SPL {
        SPL { storage: value}
    }
}

impl From<TOBArray> for SPL {
    /// Converts a TOBArray object into a TOB object.
    ///
    /// # Examples
    /// ```
    /// use epnl::SPL;
    /// let spl = SPL::from([60.0; 24]);
    /// ```    
    fn from(value: TOBArray) -> SPL {
        SPL { storage: TOBStorage::from(value)}
    }
}


impl From<&[TOBScalarType]> for SPL {
    /// Converts a TOB storage object into a TOB object.
    ///
    /// # Examples
    /// ```
    /// use epnl::SPL;
    /// let vec = Vec::from([60.0; 24]);
    /// let spl = SPL::from(&vec[..]);
    /// ```        
    fn from(value: &[TOBScalarType]) -> SPL {
        SPL { storage: TOBStorage::from(value)}        
    }
}

impl Deref for SPL {
    type Target = TOBStorage;
    fn deref(&self) -> &Self::Target {
        &self.storage
    }
}

impl DerefMut for SPL {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.storage
    }
}
