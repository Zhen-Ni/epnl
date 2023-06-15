pub const TOBSIZE: usize = 24;
pub type TOBScalarType = f64;
pub type TOBContainedType = [TOBScalarType; TOBSIZE];

/// TOB mid-band frequency for evaluating EPNL.
pub static TOBFREQ: TOB = TOB {pressure_level :
                               [50., 63., 80.,
                                100., 125., 160., 200., 250.,
                                315., 400., 500., 630., 800.,
                                1000., 1250., 1600., 2000., 2500.,
                                3150., 4000., 5000., 6300., 8000.,
                               10000.]};

/// Structure for one-third octave band (TOB) sound pressures. The
/// mid-frequency of TOB used for evaluating EPNL ranges from 50 to
/// 10000 Hz.
#[derive(Clone)]
pub struct TOB {
    /// The one-third octave band sound pressure levels.
    pub(crate) pressure_level: TOBContainedType,
}


impl TOB {
    /// Constructs a new TOB object with nan values.
    ///
    /// # Examples
    /// ```
    /// use epnl::TOB;
    /// let spl = TOB::new();
    /// let spl50 = spl.get(1);
    /// // All values in `spl` are initialized to nan.
    /// assert!(f64::is_nan(spl50));
    /// ```
    pub fn new() -> TOB {
        let pressure_level = [TOBScalarType::NAN; TOBSIZE];
        TOB{pressure_level}
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
        self.pressure_level[index - 1]
    }

    /// Set the i-th band in the TOB list to given value.  To be
    /// consistent with ICAO ANNEX 16, the index starts from 1 and
    /// ends with 24 (included).
    ///
    /// ```
    /// use epnl::TOB;
    /// let mut spl = TOB::new();
    /// spl.set(5, 75.0);
    /// let spl125 = spl.get(5);
    /// assert_eq!(spl125, 75.0);
    /// ```
    pub fn set(&mut self, index: usize, value: TOBScalarType) {
        self.pressure_level[index - 1] = value;
    }
}


impl From<TOBContainedType> for TOB {
    /// Converts an array into a TOB object.
    fn from(value: TOBContainedType) -> TOB {
        TOB {pressure_level : value}
    }    
}


impl From<&[TOBScalarType]> for TOB {
    /// Converts a slice into a TOB object.
    fn from(value: &[TOBScalarType]) -> TOB {
        let pressure_level = value.try_into().unwrap_or_else(|_v| {
            panic!("Expect a Vec of length {}, got length {} instead.", TOBSIZE, value.len())
        }
        );
            
        TOB {pressure_level}
    }    
}

impl From<Vec<TOBScalarType>> for TOB {
    /// Converts a vector into a TOB object.
    fn from(value: Vec<TOBScalarType>) -> TOB {
        let pressure_level: TOBContainedType = value.try_into().unwrap_or_else(
            |v: Vec<TOBScalarType>| {
                panic!("Expect a Vec of length {}, got length {} instead.", TOBSIZE, v.len())
            }
        );
        TOB {pressure_level}
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut spl = TOB::from([0.0; 24]);
        spl.set(5, 75.0);
        let spl125 = spl.get(5);
        // All values in `spl` are initialized to nan.
        assert_eq!(spl125, 75.0);
     }
}
