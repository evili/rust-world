use world::*;

#[test]
fn basic_world() {
    let w = World::new(2.1, 0.1);
    assert_eq!(1, w.things().len());
    let nxt : Option<&Box<dyn Thing>> = w.things().next();
    if let Some(t) = nxt {
        assert_eq!(DEFAULT_THING_ENERGY, t.energy());
        assert_eq!(DEFAULT_THING_MIN_ENERGY_SON, t.min_energy_son());
        assert_eq!(DEFAULT_THING_ENERGY_SON, t.energy_son());        
    }
    else {
                panic!("No Things found!");
    }
    let n = match w.step(2) {
        Some(s) => s,
        None => 0,
    };
    assert_eq!(2, n);
    assert_eq!(2, w.things().len());
}
