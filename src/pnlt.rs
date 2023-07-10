use std::ops::{Deref, DerefMut};
use crate::{TOBStorage, TOBSIZE, SPL, TOBScalarType, TOBFREQ, PNL};


#[derive(Debug, Clone)]
pub struct PNLT {
    storage: TOBScalarType
}


impl From<TOBScalarType> for PNLT {
    /// Converts a TOBStorage object into a PNLT object.
    fn from(value: TOBScalarType) -> Self {
        PNLT { storage: value}
    }
}


impl From<SPL> for PNLT {
    /// Converts a SPL object into a PNLT object.
    ///
    /// # Examples
    /// ```
    /// use epnl::{SPL, PNLT};
    /// let spl = SPL::from([60.0; 24]);
    /// let pnlt = PNLT::from(spl);
    /// ```        
    fn from(spl: SPL) -> Self {
        let slope = get_slope(&spl);
        let encircled_slope = encircle_slope(&slope);
        let encircled_spl = encircle_spl(&slope, &encircled_slope);
        let aspl = get_adjusted_spl(&spl, &slope, &encircled_spl);
        let slope = get_new_slope(&aspl);
        let sb = get_averaged_slope(&slope);
        let fspl = get_final_spl(&spl, &sb);
        let f = get_differences(&spl, &fspl);
        let c = get_tone_correction(&f);
        let pnl = PNL::from(spl);
        get_correction_factor(&pnl, &c)
    }
}    


impl Deref for PNLT {
    type Target = TOBScalarType;
    fn deref(&self) -> &Self::Target {
        &self.storage
    }
}

impl DerefMut for PNLT {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.storage
    }
}



// Step 1: Get slope defined in Annex 16- Volume I, APP 2-16.
fn get_slope(spl: &SPL) -> TOBStorage {
    let mut s = TOBStorage::from([0.0; TOBSIZE]);
    for i in 4..=24 {
        s.set(i, spl.get(i) - spl.get(i-1));
    }
    s
}


// Step 2
fn encircle_slope(slope: &TOBStorage) -> TOBStorage {
    let mut encircled_array = TOBStorage::from([0.0; TOBSIZE]);    
    for i in 2..=24 {
        let diff = slope.get(i) - slope.get(i-1);
        if diff.abs() > 5. {
            encircled_array.set(i, diff);
        }
    }
    encircled_array
}


// Step 3: Get encircled values of the slope.
fn encircle_spl(slope: &TOBStorage, encircled_slope: &TOBStorage) -> TOBStorage {
    let mut encircled_array = TOBStorage::from([0.0; TOBSIZE]);
    for i in 4..=24 {
        // Condition a)
        if encircled_slope.get(i) > slope.get(i-1).max(0.0) {
            encircled_array.set(i, 1.0)
        }
        // Condition b)
        if (encircled_slope.get(i) <= 0.0) && (slope.get(i-1) > 0.0)  {
            encircled_array.set(i - 1, 1.0)
        }
    }
    encircled_array
}


// Step 4: 4.3.1 in Annex 16 - Volume I, App 2-17.
fn get_adjusted_spl(spl: &SPL, slope: &TOBStorage, encircled_spl: &TOBStorage) -> SPL {
    let mut aspl = spl.clone();
    for i in 2..=23 {
        if encircled_spl.get(i) > 0.0 {
            aspl.set(i, (spl.get(i-1) + spl.get(i+1)) / 2.);
        }
    }
    if encircled_spl.get(24) > 0.0 {
        aspl.set(24, spl.get(23) + slope.get(23));
    }
    aspl
}


// Step 5: Recompute new slope
fn get_new_slope(aspl: &SPL) -> [TOBScalarType; TOBSIZE + 1] {
    let mut s = [0.0; TOBSIZE + 1];
    for i in 3..24 {
        s[i] = aspl[i] - aspl[i-1];
    }
    s[2] = s[3];
    s[24] = s[23];
    s
}


// Step 6: Get averaged slope
fn get_averaged_slope(s: &[TOBScalarType; TOBSIZE + 1]) -> TOBStorage  {
    let mut sb = TOBStorage::from([0.0; TOBSIZE]);
    for i in 1..TOBSIZE-1 {
        sb[i] = (s[i-1] + s[i] + s[i+1])/ 3.;
    }
    sb
}


// Step 7
fn get_final_spl(spl: &SPL, sb: &TOBStorage) -> SPL {
    let mut fspl = SPL::from([0.0; TOBSIZE]);
    fspl.set(3, spl.get(3));
    for i in 4..=24 {
        fspl.set(i, spl.get(i-1) + sb.get(i-1));
    }
    fspl
}


// Step 8: Calculate differences
fn get_differences(spl: &SPL, fspl: &SPL) -> SPL {
    let res: Vec<_> = spl.iter().zip(***fspl).map(|(&lhs, rhs)| {lhs - rhs}).collect();
    SPL::from(&res[..])
}


// Step 9
fn get_tone_correction(upper_f: &SPL) -> SPL {
    let mut c = SPL::from([0.0; TOBSIZE]);    
    for i in 3..=24 {
        let f = TOBFREQ.get(i);
        let fi = upper_f.get(i);
        c.set(i,
              if (f < 500.) || (f > 5000.) {
                  if fi < 1.5 { 0.0 }
                  else if fi < 3. { fi / 3. - 0.5 }
                  else if fi < 20. { fi / 6. }
                  else { 3. + 1. / 3.}
              }
              else {
                  if fi < 1.5 { 0.0 }
                  else if fi < 3. { fi / 3. - 0.5 }
                  else if fi < 20. { fi / 6. }
                  else { 3. + 1. / 3.}              
              }
        )
    }
    c
}


// Step 10
fn get_correction_factor(pnl: &PNL, c: &SPL) ->  PNLT {
    PNLT::from(**pnl + c.into_iter().reduce(TOBScalarType::max).unwrap())
}


