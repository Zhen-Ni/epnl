use std::ops::{Deref, DerefMut};
use crate::{TOBStorage};

/// TOB mid-band frequency for evaluating EPNL.
pub static TOBFREQ: Freq = Freq {frequency: TOBStorage { storage :
                                [50., 63., 80.,
                                 100., 125., 160., 200., 250.,
                                 315., 400., 500., 630., 800.,
                                 1000., 1250., 1600., 2000., 2500.,
                                 3150., 4000., 5000., 6300., 8000.,
                                 10000.]}};

/// Structure for one-third octave band (TOB) sound pressures. The
/// mid-frequency of TOB used for evaluating EPNL ranges from 50 to
/// 10000 Hz.
#[derive(Clone)]
pub struct Freq {
    /// The one-third octave band sound pressure levels.
    frequency: TOBStorage,
}

impl From<TOBStorage> for Freq {
    /// Converts an array into a Freq object.
    fn from(value: TOBStorage) -> Freq {
        Freq { frequency: value}
    }
}

impl Deref for Freq {
    type Target = TOBStorage;
    fn deref(&self) -> &Self::Target {
        &self.frequency
    }
}

impl DerefMut for Freq {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.frequency
    }
}
