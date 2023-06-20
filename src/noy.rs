
use std::ops::{Deref, DerefMut};
use crate::{TOBStorage, TOBScalarType, SPL, TOBSIZE};


static TABLE: [[TOBScalarType; 9]; TOBSIZE] = [
    [ 91.0                    , 64.0 , 52.0 , 49.0 , 55.0 , 0.043478 ,           0.030103 , 0.079520 , 0.058098 ],
    [ 85.9                    , 60.0 , 51.0 , 44.0 , 51.0 , 0.040570 ,           0.030103 , 0.068160 , 0.058098 ],
    [ 87.3                    , 56.0 , 49.0 , 39.0 , 46.0 , 0.036831 ,           0.030103 , 0.068160 , 0.052288 ],
    [ 79.0                    , 53.0 , 47.0 , 34.0 , 42.0 , 0.036831 ,           0.030103 , 0.059640 , 0.047534 ],
    [ 79.8                    , 51.0 , 46.0 , 30.0 , 39.0 , 0.035336 ,           0.030103 , 0.053013 , 0.043573 ],
    [ 76.0                    , 48.0 , 45.0 , 27.0 , 36.0 , 0.033333 ,           0.030103 , 0.053013 , 0.043573 ],
    [ 74.0                    , 46.0 , 43.0 , 24.0 , 33.0 , 0.033333 ,           0.030103 , 0.053013 , 0.040221 ],
    [ 74.9                    , 44.0 , 42.0 , 21.0 , 30.0 , 0.032051 ,           0.030103 , 0.053013 , 0.037349 ],
    [ 94.6                    , 42.0 , 41.0 , 18.0 , 27.0 , 0.030675 ,           0.030103 , 0.053013 , 0.034859 ],
    [ TOBScalarType::INFINITY , 40.0 , 40.0 , 16.0 , 25.0 , 0.030103 , TOBScalarType::NAN , 0.053013 , 0.034859 ],
    [ TOBScalarType::INFINITY , 40.0 , 40.0 , 16.0 , 25.0 , 0.030103 , TOBScalarType::NAN , 0.053013 , 0.034859 ],
    [ TOBScalarType::INFINITY , 40.0 , 40.0 , 16.0 , 25.0 , 0.030103 , TOBScalarType::NAN , 0.053013 , 0.034859 ],
    [ TOBScalarType::INFINITY , 40.0 , 40.0 , 16.0 , 25.0 , 0.030103 , TOBScalarType::NAN , 0.053013 , 0.034859 ],
    [ TOBScalarType::INFINITY , 40.0 , 40.0 , 16.0 , 25.0 , 0.030103 , TOBScalarType::NAN , 0.053013 , 0.034859 ],
    [ TOBScalarType::INFINITY , 38.0 , 38.0 , 15.0 , 23.0 , 0.030103 , TOBScalarType::NAN , 0.059640 , 0.034859 ],
    [ TOBScalarType::INFINITY , 34.0 , 34.0 , 12.0 , 21.0 , 0.029960 , TOBScalarType::NAN , 0.053013 , 0.040221 ],
    [ TOBScalarType::INFINITY , 32.0 , 32.0 , 9.00 , 18.0 , 0.029960 , TOBScalarType::NAN , 0.053013 , 0.037349 ],
    [ TOBScalarType::INFINITY , 30.0 , 30.0 , 5.00 , 15.0 , 0.029960 , TOBScalarType::NAN , 0.047712 , 0.034859 ],
    [ TOBScalarType::INFINITY , 29.0 , 29.0 , 4.00 , 14.0 , 0.029960 , TOBScalarType::NAN , 0.047712 , 0.034859 ],
    [ TOBScalarType::INFINITY , 29.0 , 29.0 , 5.00 , 14.0 , 0.029960 , TOBScalarType::NAN , 0.053013 , 0.034859 ],
    [ TOBScalarType::INFINITY , 30.0 , 30.0 , 6.00 , 15.0 , 0.029960 , TOBScalarType::NAN , 0.053013 , 0.034859 ],
    [ TOBScalarType::INFINITY , 31.0 , 31.0 , 10.0 , 17.0 , 0.029960 ,           0.029960 , 0.068160 , 0.037349 ],
    [ 44.3                    , 37.0 , 34.0 , 17.0 , 23.0 , 0.042285 ,           0.029960 , 0.079520 , 0.037349 ],
    [ 50.7                    , 41.0 , 37.0 , 21.0 , 29.0 , 0.042285 ,           0.029960 , 0.059640 , 0.043573 ]];


#[derive(Debug, Clone)]
pub struct Noy {
    storage: TOBStorage
}

impl From<TOBStorage> for Noy {
    /// Converts a TOBStorage object into a Noy object.
    fn from(value: TOBStorage) -> Noy {
        Noy { storage: value}
    }
}

impl From<SPL> for Noy {
    /// Converts SPL into perceived noisiness.
    ///
    /// # Examples
    /// ```
    /// use epnl::{Noy, SPL};
    /// let spl = SPL::from([60.0; 24]);
    /// let noy = Noy::from(spl);
    /// let error = noy.get(1) - 0.5856137;
    /// assert!(error.abs() < 0.1);
    /// ```
    fn from(spl: SPL) -> Noy {
        let mut storage = TOBStorage::new();
        for ((ni, row), spli) in storage.iter_mut().zip(TABLE).zip(**spl) {
            let [spla, splb, splc, spld, sple, mb, mc, md, me] = row;
            *ni = if spli >= spla {
                (10.0 as TOBScalarType).powf(mc * (spli - splc))
            } else if spli >= splb {
                (10.0 as TOBScalarType).powf(mb * (spli - splb))
            } else if spli >= sple {
                0.3 * (10.0 as TOBScalarType).powf(me * (spli - sple))
            } else {
                0.1 * (10.0 as TOBScalarType).powf(md * (spli - spld))
            };
        }
        Noy::from(storage)
    }
}

impl Deref for Noy {
    type Target = TOBStorage;
    fn deref(&self) -> &Self::Target {
        &self.storage
    }
}

impl DerefMut for Noy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.storage
    }
}

