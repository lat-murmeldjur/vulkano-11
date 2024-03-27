// experimental generic composite reductive approximation outline
use std::sync::mpsc;
use std::thread;


use crate::f32_3::dd_f32_3;
use crate::f64_3::mltply_f64_3;
use crate::magma_ocean::{magma, petrify, Stone};
use crate::positions::{move_positions};

pub static ts_f64: f64 = 5.391247 * 1e-44;
pub static ls_f64: f64 = 299792458.0 * 1000000000.0 * 6.1879273537329 * 1e+25;

pub struct Anomaly {
    pub Anomaly: Vec<Anomaly>,
    pub Component: Vec<Component>,
    pub Force: Vec<Force>,
}

pub struct Composition {
    pub Space: Vec<[f32; 3]>,
    pub Distribution: Vec<fn(Vec<[f32; 3]>) -> Vec<[f32; 3]>>,
}

pub struct Component {
    pub Component: Vec<Component>,
    pub Composition: Vec<Composition>,
    pub Property: Vec<Property>,
}

pub struct Property {
    pub Name: f64,
    pub Value: f64,
}

pub struct Force {
    pub Force: Vec<Force>,
    pub Range: Vec<f64>,
    pub Domain: Vec<Component>,
}

pub fn interact(anom: &mut Anomaly) {
    thread::scope(|s| {
        let mut handles: Vec<thread::ScopedJoinHandle<()>> = vec![];
        for mut a in anom.Anomaly.iter_mut() {
            let handle = s.spawn(move || interact(&mut a));
            handles.push(handle);
        }
        for h in handles {
            h.join().unwrap();
        }
    });

    component_interact(anom);
}

pub fn component_interact(_anom: &mut Anomaly) {
    //    for f in &anom.Force {
    //
    //    }
}

pub fn progress(anom: &mut Anomaly, time: f64) {
    thread::scope(|s| {
        let mut handles: Vec<thread::ScopedJoinHandle<()>> = vec![];
        for mut a in anom.Anomaly.iter_mut() {
            let handle = s.spawn(move || progress(&mut a, time));
            handles.push(handle);
        }
        for h in handles {
            h.join().unwrap();
        }
    });

    let steps = (time / ts_f64) as u64;
    for _ in 0..steps {
        thread::scope(|s| {
            let mut handles: Vec<thread::ScopedJoinHandle<()>> = vec![];
            for mut c in anom.Component.iter_mut() {
                let handle = s.spawn(move || {
                    component_progress(&mut c, ts_f64);
                });
                handles.push(handle);
            }
            for h in handles {
                h.join().unwrap();
            }
        });
    }
}

pub fn component_property(component: &mut Component, name: f64) -> f64 {
    let prop: Vec<&mut Property> = component
        .Property
        .iter_mut()
        .filter(|c| c.Name == name)
        .collect();

    return prop[0].Value;
}

pub fn component_progress(component: &mut Component, time: f64) {
    for mut c in component.Component.iter_mut() {
        component_progress(&mut c, time);
    }

    let inertia_0 = component_property(component, In0);
    let inertia_1 = component_property(component, In1);
    let inertia_2 = component_property(component, In2);

    for c in &mut component.Composition {
        for s in c.Space.iter_mut() {
            let mov0 = mltply_f64_3([inertia_0, inertia_1, inertia_2], ts_f64);
            *s = dd_f32_3(*s, [mov0[0] as f32, mov0[1] as f32, mov0[2] as f32]);
        }
    }
}

pub fn view(anom: &mut Anomaly) -> Vec<Stone> {
    let mut ret: Vec<Stone> = vec![];
    let mut rs: Vec<mpsc::Receiver<Vec<Stone>>> = vec![];

    thread::scope(|s| {
        for mut a in anom.Anomaly.iter_mut() {
            let (tx, rx) = mpsc::channel();
            rs.push(rx);
            s.spawn(move || {
                let k = view(&mut a);
                tx.send(k).unwrap();
            });
        }
    });

    for c in anom.Component.iter_mut() {
        ret.append(&mut component_view(c));
    }

    for r in rs {
        let mut rec = r.recv().unwrap();
        ret.append(&mut rec);
    }

    ret
}

pub fn component_view(component: &mut Component) -> Vec<Stone> {
    let mut ret: Vec<Stone> = vec![];

    for c in component.Component.iter_mut() {
        ret.append(&mut component_view(c));
    }

    let size = component_property(component, Ms);

    for c in &component.Composition {
        for d in &c.Distribution {
            for v in &d(c.Space.clone()) {
                let mut s = petrify(magma(2, size as f32));
                move_positions(&mut s.positions, *v);
                ret.push(s);
            }
        }
    }
    ret
}

pub fn add_particle(anom: &mut Anomaly, position: [f32; 3], properties: Vec<Property>) {
    anom.Anomaly.push(particle(position, properties));
}

pub fn add_particle_by(anom: &mut Anomaly, p: Anomaly) {
    anom.Anomaly.push(p);
}

pub fn particle(position: [f32; 3], properties: Vec<Property>) -> Anomaly {
    let anom = Anomaly {
        Anomaly: vec![],
        Component: vec![Component {
            Component: vec![],
            Composition: vec![Composition {
                Space: vec![position],
                Distribution: vec![particular],
            }],
            Property: properties,
        }],
        Force: force_base().Force,
    };

    anom
}

//

//

//

// future ref example

static EC: f64 = 313.0;
static Sp: f64 = 591.0;
static Ms: f64 = 343.0;
static Cr: f64 = 0.10;
static In0: f64 = 141.0;
static In1: f64 = 141.1;
static In2: f64 = 141.2;
static QMs: [f64; 6] = [2.2, 4.7, 1.28, 96.0, 173.1, 4.18];

pub fn e(position: [f32; 3], inertia: [f64; 3], clock: bool) -> Anomaly {
    let sp = if clock { 0.5 } else { -0.5 };
    particle(
        position,
        vec![
            Property {
                Name: Sp,
                Value: sp,
            },
            Property {
                Name: EC,
                Value: -1.0,
            },
            Property {
                Name: Ms,
                Value: 0.511,
            },
            Property {
                Name: In0,
                Value: inertia[0],
            },
            Property {
                Name: In1,
                Value: inertia[1],
            },
            Property {
                Name: In2,
                Value: inertia[2],
            },
        ],
    )
}

pub fn q(
    position: [f32; 3],
    inertia: [f64; 3],
    clock: bool,
    charge: bool,
    color: u8,
    flavor: u8,
) -> Anomaly {
    let sp = if clock { 0.5 } else { -0.5 };
    let ch = if charge { 2.0 / 3.0 } else { -1.0 / 3.0 };

    particle(
        position,
        vec![
            Property {
                Name: Sp,
                Value: sp,
            },
            Property {
                Name: EC,
                Value: ch,
            },
            Property {
                Name: Ms,
                Value: QMs[(flavor % 6) as usize],
            },
            Property {
                Name: Cr,
                Value: (color % 6) as f64,
            },
            Property {
                Name: In0,
                Value: inertia[0],
            },
            Property {
                Name: In1,
                Value: inertia[1],
            },
            Property {
                Name: In2,
                Value: inertia[2],
            },
        ],
    )
}

pub fn particular(coordinates: Vec<[f32; 3]>) -> Vec<[f32; 3]> {
    return coordinates;
}

pub fn force_base() -> Force {
    return Force {
        Force: vec![
            // S
            Force {
                Force: vec![],
                Range: vec![1e-15],
                Domain: vec![Component {
                    Component: vec![],
                    Composition: vec![],
                    Property: vec![Property {
                        Name: Cr,
                        Value: 1.0,
                    }],
                }],
            },
            // EM
            Force {
                Force: vec![],
                Range: vec![f64::MAX],
                Domain: vec![Component {
                    Component: vec![],
                    Composition: vec![],
                    Property: vec![Property {
                        Name: EC,
                        Value: 1.0 / 137.0,
                    }],
                }],
            },
            // W
            Force {
                Force: vec![
                    // N
                    Force {
                        Force: vec![],
                        Range: vec![1e-18],
                        Domain: vec![Component {
                            Component: vec![],
                            Composition: vec![],
                            Property: vec![Property {
                                Name: Ms,
                                Value: 1e-13,
                            }],
                        }],
                    },
                    // C
                    Force {
                        Force: vec![],
                        Range: vec![1e-18],
                        Domain: vec![Component {
                            Component: vec![],
                            Composition: vec![],
                            Property: vec![Property {
                                Name: Sp,
                                Value: 1e-13,
                            }],
                        }],
                    },
                ],
                Range: vec![],
                Domain: vec![],
            },
            // G
            Force {
                Force: vec![],
                Range: vec![f64::MAX],
                Domain: vec![Component {
                    Component: vec![],
                    Composition: vec![],
                    Property: vec![Property {
                        Name: Ms,
                        Value: 1e-41,
                    }],
                }],
            },
        ],
        Range: vec![],
        Domain: vec![],
    };
}
