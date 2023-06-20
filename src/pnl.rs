use std::ops::{Deref, DerefMut};
use crate::{Noy, TOBScalarType, SPL};

#[derive(Debug, Clone)]
pub struct PNL {
    storage: TOBScalarType
}


impl From<TOBScalarType> for PNL {
    /// Converts a TOBStorage object into a PNL object.
    fn from(value: TOBScalarType) -> Self {
        PNL { storage: value}
    }
}

impl From<Noy> for PNL {
    /// Converts perceived noisiness value into perceived noise level (PNL).
    fn from(value: Noy) -> Self {
        let max = value.into_iter().reduce(TOBScalarType::max).expect("Empty Noy??");
        let sum: TOBScalarType = value.iter().sum();
        let tpn = 0.85 * max + 0.15 * sum;
        PNL::from(40.0 + 10.0 * tpn.log2())
    }
}

impl From<SPL> for PNL {
    /// Converts SPL into PNL.
    ///
    /// # Examples
    /// ```
    /// use epnl::{SPL, PNL};
    /// let spl = SPL::from([60.0; 24]);
    /// let pnl = PNL::from(spl);
    /// ```        

    fn from(value: SPL) -> Self {
        PNL::from(Noy::from(value))
    }
}
    
impl Deref for PNL {
    type Target = TOBScalarType;
    fn deref(&self) -> &Self::Target {
        &self.storage
    }
}

impl DerefMut for PNL {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.storage
    }
}

