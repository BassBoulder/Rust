// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration { s : u64 }

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {s: s}
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.s as f64 / Self::period()
    }

    fn period() -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

const EARTH_PERIOD : f64 = 31_557_600.0;

impl Planet for Mercury     { fn period() -> f64 { EARTH_PERIOD * 0.2408467} }
impl Planet for Venus       { fn period() -> f64 { EARTH_PERIOD * 0.61519726} }
impl Planet for Earth       { fn period() -> f64 { EARTH_PERIOD } }
impl Planet for Mars        { fn period() -> f64 { EARTH_PERIOD * 1.8808158} }
impl Planet for Jupiter     { fn period() -> f64 { EARTH_PERIOD * 11.862615} }
impl Planet for Saturn      { fn period() -> f64 { EARTH_PERIOD * 29.447498} }
impl Planet for Uranus      { fn period() -> f64 { EARTH_PERIOD * 84.016846} }
impl Planet for Neptune     { fn period() -> f64 { EARTH_PERIOD * 164.79132} }
