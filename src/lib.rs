/// # world
/// Simulate a world with living things
/// 
use std::slice::Iter;
use std::rc::Rc;

pub const DEFAULT_SUN_POWER: f64  = 2.1;
pub const DEFAULT_ENTROPY: f64  = 0.1;
pub const MIN_SUN_POWER: f64 = 0.0;
pub const MIN_ENTROPY: f64 = 0.0;
pub const MAX_ENTROPY: f64 = 1.0;

pub const DEFAULT_THING_ENERGY: f64 = 20.0;
pub const DEFAULT_THING_MIN_ENERGY_SON: f64 = 21.0;
pub const DEFAULT_THING_ENERGY_SON: f64 = 0.5;

pub struct World {
    sun_power: f64,
    entropy: f64,
    things: Vec<Box<dyn Thing>>,
}

pub trait Thing {
    fn energy(&self) -> f64;
    fn min_energy_son(&self) -> f64;
    fn energy_son(&self) -> f64;
}

pub struct Moss {
    world: Rc<World>,
    energy: f64,
    min_energy_son: f64,
    energy_son: f64
}

impl Moss {
    fn new(world: &World, energy: f64, min_energy_son: f64, energy_son: f64) -> Moss {
        Moss {
            world: Rc::new(world),
            energy,
            min_energy_son,
            energy_son
        }
    }
}

impl Thing for Moss {
    fn energy(&self) -> f64 {
       self.energy 
    }
    fn min_energy_son(&self) -> f64 {
        self.min_energy_son
    }
    fn energy_son(&self) -> f64 {
        self.energy_son
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

        let t: Vec<Box<dyn Thing>> = vec![];

        World {
            sun_power,
            entropy,
            things: t,
        }
    }

    pub fn add_first(&self) {
        self.things.push(
            Box::new(
                Moss::new(
                    self,
                    DEFAULT_THING_ENERGY,
                    DEFAULT_THING_MIN_ENERGY_SON,
                    DEFAULT_THING_ENERGY_SON
                )
            )
        );
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
    #[should_panic(expected="Entropy must be lower than 1: 1")]
    fn entropy_max_world() {
        let w = World::new(DEFAULT_SUN_POWER, MAX_ENTROPY);
        w.entropy();
    }
    #[test]
    #[should_panic(expected="Sun Power must be greater than 0: 0")]
    fn sun_power_min_world() {
        let w = World::new(MIN_SUN_POWER, DEFAULT_ENTROPY);
        w.entropy();
    }
    #[test]
    fn new_thing() {
        let w = World::new(DEFAULT_SUN_POWER, DEFAULT_ENTROPY);
        let m = Moss::new(&w, DEFAULT_THING_ENERGY, DEFAULT_THING_MIN_ENERGY_SON, DEFAULT_THING_ENERGY_SON);
        assert_eq!(m.energy(), DEFAULT_THING_ENERGY);
        assert_eq!(m.min_energy_son(), DEFAULT_THING_MIN_ENERGY_SON);
        assert_eq!(m.energy_son(), DEFAULT_THING_ENERGY_SON);
    }
    #[test]
    #[should_panic(expected="Energy must be greater than 0: 0")]
    fn energy_thing_min() {
        let w = World::new(DEFAULT_SUN_POWER, DEFAULT_ENTROPY);
        let m = Moss::new(&w, 0.0, DEFAULT_THING_MIN_ENERGY_SON, DEFAULT_THING_ENERGY_SON);
        m.energy();
    }
    #[test]
    #[should_panic(expected="Min Energy Son must be greater than 0: 0")]
    fn min_energy_son_thing_min() {
        let w = World::new(DEFAULT_SUN_POWER, DEFAULT_ENTROPY);
        let m = Moss::new(&w, DEFAULT_THING_ENERGY, 0.0, DEFAULT_THING_ENERGY_SON);
        m.min_energy_son();
    }
    #[test]
    #[should_panic(expected="Energy Son must be greater than 0: 0")]
    fn energy_son_thing_min() {
        let w = World::new(DEFAULT_SUN_POWER, DEFAULT_ENTROPY);
        let m = Moss::new(&w, DEFAULT_THING_ENERGY, DEFAULT_THING_MIN_ENERGY_SON, 0.0);
        m.energy_son();
    }
    #[test]
    #[should_panic(expected="Energy Son must be lower than 1: 1")]
    fn energy_son_thing_max() {
        let w = World::new(DEFAULT_SUN_POWER, DEFAULT_ENTROPY);
        let m = Moss::new(&w, DEFAULT_THING_ENERGY, DEFAULT_THING_MIN_ENERGY_SON, 1.0);
        m.energy_son();
    }
    #[test]
    #[should_panic(expected="Energy Son must be high enough: 0.1")]
    fn energy_son_thing_consistent() {
        // son.energy = parent.min_energy_son * parent.energy_son * (1-parent.world.entropy())
        // son.energy >= 1.0 or it will die immediately.
        // This implies:
        // parent.min_energy_son * parent.energy_son * (1-parent.world.entropy()) >= 1
        // parent.energy_son >= 1/(parent.min_energy_son * (1-parent.world.entropy())
        let w = World::new(DEFAULT_SUN_POWER, DEFAULT_ENTROPY);
        let bad_ene_son = 1.0 / ((1.0 - w.entropy()) * DEFAULT_THING_MIN_ENERGY_SON);
        println!("Bad Energy Son calculated as: {}", bad_ene_son);
        let m = Moss::new(&w, DEFAULT_THING_ENERGY, DEFAULT_THING_MIN_ENERGY_SON, bad_ene_son);
        m.energy_son();
    }
}
