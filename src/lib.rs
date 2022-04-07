/// # world
/// Simulate a world with living things
/// 
use std::slice::Iter;

pub const DEFAULT_SUN_POWER: f64  = 2.1;
pub const DEFAULT_ENTROPY: f64  = 0.1;
pub const MIN_SUN_POWER: f64 = 0.0;
pub const MIN_ENTROPY: f64 = 0.0;
pub const MAX_ENTROPY: f64 = 1.0;


pub struct World {
    sun_power: f64,
    entropy: f64,
    things: Vec<Box<dyn Thing>>,
    _dead_things: Vec<Box<dyn Thing>>
}
pub trait Thing {
    fn energy(&self) -> f64;
    fn min_ene_son(&self) -> f64;
    fn ene_son(&self) -> f64;
}
pub struct Moss {}

impl Thing for Moss {
    fn energy(&self) -> f64 {
        10.0
    }
    fn min_ene_son(&self) -> f64 {
        10.0
    }
    fn ene_son(&self) -> f64 {
        0.5
    }
}

impl World {

    pub fn new(sun_power: f64, entropy: f64) -> World {
        if sun_power <= MIN_SUN_POWER {
            panic!("Sun Power must be greater than {}: {}", MIN_SUN_POWER,  sun_power);
        }
        if entropy <= MIN_ENTROPY {
            panic!("Entropy must be greater than {}: {}", MIN_ENTROPY,  entropy);
        }
        if entropy >= MAX_ENTROPY {
            panic!("Entropy must be less than {}: {}", MAX_ENTROPY,  entropy);
        }

        let t: Vec<Box<dyn Thing>> = vec![Box::new(Moss{})];
        let dt: Vec<Box<dyn Thing>> = vec![];

        World{
            sun_power,
            entropy,
            things: t,
            _dead_things: dt
        }
    }

    pub fn sun_power(&self) -> f64 { self.sun_power }

    pub fn entropy(&self) -> f64 { self.entropy }
    pub fn things(&self) -> Iter<Box<dyn Thing>> {
        self.things.iter()
    }
    pub fn step(&self, _n_steps : u32) -> Option<u32> {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn new_world() {
        let w = World::new(DEFAULT_SUN_POWER, DEFAULT_ENTROPY);
        assert_eq!(w.sun_power(), DEFAULT_SUN_POWER);
        assert_eq!(w.entropy(), DEFAULT_ENTROPY);
    }
    #[test]
    #[should_panic(expected="Entropy must be greater than 0: 0")]
    fn entropy_min_world() {
        let w = World::new(DEFAULT_SUN_POWER, MIN_ENTROPY);
        w.entropy();
    }
    #[test]
    #[should_panic]
    fn entropy_max_world() {
        let w = World::new(DEFAULT_SUN_POWER, MAX_ENTROPY);
        w.entropy();
    }
    #[test]
    #[should_panic]
    fn sun_power_min_world() {
        let w = World::new(MIN_SUN_POWER, DEFAULT_ENTROPY);
        w.entropy();
    }

}
