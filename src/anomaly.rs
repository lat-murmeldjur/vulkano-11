// experimental generic composite reductive approximation outline
use rand::rngs::ThreadRng;
use std::sync::mpsc;
use std::thread;

use crate::f32_3::dd_f32_3;
use crate::f64_3::mltply_f64_3;
use crate::gen_f64_3;
use crate::magma_ocean::{magma, petrify, Stone};
use crate::nrmlz_f64_3;
use crate::positions::move_positions;

pub static TS_F64: f64 = 5.391247 * 1e-44;
pub static LS_F64: f64 = 299792458.0 * 1000000000.0 * 6.1879273537329 * 1e+25;

pub struct Anomaly {
    pub anomaly: Vec<Anomaly>,
    pub component: Vec<Component>,
    pub force: Vec<Force>,
}

pub struct Composition {
    pub space: Vec<[f32; 3]>,
    pub distribution: Vec<fn(Vec<[f32; 3]>) -> Vec<[f32; 3]>>,
}

pub struct Component {
    pub component: Vec<Component>,
    pub composition: Vec<Composition>,
    pub property: Vec<Property>,
}

pub struct Property {
    pub name: f64,
    pub value: f64,
}

pub struct Force {
    pub force: Vec<Force>,
    pub range: Vec<f64>,
    pub domain: Vec<Component>,
}

pub fn interact(anom: &mut Anomaly) {
    thread::scope(|s| {
        let mut handles: Vec<thread::ScopedJoinHandle<()>> = vec![];
        for mut a in anom.anomaly.iter_mut() {
            let handle = s.spawn(move || interact(&mut a));
            handles.push(handle);
        }
        for h in handles {
            h.join().unwrap();
        }
    });

    let mut rng = rand::thread_rng();

    for i in 0..anom.anomaly.len() {
        for j in 0..anom.anomaly.len() {
            let (e, f) = if i < j {
                // `i` is in the left half
                let (left, right) = anom.anomaly.split_at_mut(j);
                (&mut left[i], &mut right[0])
            } else if i == j {
                // cannot obtain two mutable references to the
                // same element
                continue;
            } else {
                // `i` is in the right half
                let (left, right) = anom.anomaly.split_at_mut(i);
                (&mut right[0], &mut left[j])
            };
            anomaly_2_interact(e, f, &mut rng);
        }
    }

    component_interact(anom, &mut rng);
}

pub fn anomaly_2_interact(a: &mut Anomaly, b: &mut Anomaly, mut rng: &mut ThreadRng) {
    for i in a.anomaly.iter_mut() {
        for j in b.anomaly.iter_mut() {
            anomaly_2_interact(i, j, &mut rng);
        }
    }

    for df in &a.force {
        for i in 0..a.component.len() {
            for j in 0..b.component.len() {
                component_2_interact(df, &mut a.component[i], &mut b.component[j], &mut rng);
            }
        }
    }
}

pub fn component_interact(_anom: &mut Anomaly, rng: &mut ThreadRng) {
    for df in &_anom.force {
        for i in 0.._anom.component.len() {
            for j in 0.._anom.component.len() {
                let (e, f) = if i < j {
                    // `i` is in the left half
                    let (left, right) = _anom.component.split_at_mut(j);
                    (&mut left[i], &mut right[0])
                } else if i == j {
                    // cannot obtain two mutable references to the
                    // same element
                    continue;
                } else {
                    // `i` is in the right half
                    let (left, right) = _anom.component.split_at_mut(i);
                    (&mut right[0], &mut left[j])
                };
                component_2_interact(df, e, f, rng);
            }
        }
    }
}

pub fn component_2_interact(
    df: &Force,
    a: &mut Component,
    b: &mut Component,
    mut rng: &mut ThreadRng,
) {
    for i in a.component.iter_mut() {
        for j in b.component.iter_mut() {
            component_2_interact(df, i, j, &mut rng);
        }
    }

    force_apply(df, a, b, &mut rng);
}

pub fn force_apply(_f: &Force, a: &mut Component, b: &mut Component, mut rng: &mut ThreadRng) {
    //    a1 = component_property(a, IN0);
    //    a2 = component_property(a, IN1);
    //    a3 = component_property(a, IN2);
    //
    //    b1 = component_property(b, IN0);
    //    b2 = component_property(b, IN1);
    //    b3 = component_property(b, IN2);

    set_inertia(
        mltply_f64_3(nrmlz_f64_3(gen_f64_3(0.0, 10.0, &mut rng)), LS_F64),
        a,
    );

    set_inertia(
        mltply_f64_3(nrmlz_f64_3(gen_f64_3(0.0, 10.0, &mut rng)), LS_F64),
        b,
    );
}

pub fn progress(anom: &mut Anomaly, time: f64) {
    thread::scope(|s| {
        let mut handles: Vec<thread::ScopedJoinHandle<()>> = vec![];
        for mut a in anom.anomaly.iter_mut() {
            let handle = s.spawn(move || {
                interact(&mut a);
                progress(&mut a, time);
            });
            handles.push(handle);
        }
        for h in handles {
            h.join().unwrap();
        }
    });

    let mut rng = rand::thread_rng();

    for i in 0..anom.anomaly.len() {
        for j in 0..anom.anomaly.len() {
            let (e, f) = if i < j {
                // `i` is in the left half
                let (left, right) = anom.anomaly.split_at_mut(j);
                (&mut left[i], &mut right[0])
            } else if i == j {
                // cannot obtain two mutable references to the
                // same element
                continue;
            } else {
                // `i` is in the right half
                let (left, right) = anom.anomaly.split_at_mut(i);
                (&mut right[0], &mut left[j])
            };
            anomaly_2_interact(e, f, &mut rng);
        }
    }

    let steps = (time / TS_F64) as u64;
    for _ in 0..steps {
        thread::scope(|s| {
            let mut handles: Vec<thread::ScopedJoinHandle<()>> = vec![];
            for mut c in anom.component.iter_mut() {
                let handle = s.spawn(move || {
                    component_progress(&mut c, TS_F64);
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
        .property
        .iter_mut()
        .filter(|c| c.name == name)
        .collect();

    return prop[0].value;
}

pub fn set_component_property(n: f64, s: f64, component: &mut Component) {
    for p in component.property.iter_mut() {
        if n == p.name {
            *p = Property { name: n, value: s };
        }
    }
}

pub fn component_progress(component: &mut Component, time: f64) {
    for mut c in component.component.iter_mut() {
        component_progress(&mut c, time);
    }

    let inertia_0 = component_property(component, IN0);
    let inertia_1 = component_property(component, IN1);
    let inertia_2 = component_property(component, IN2);

    for c in &mut component.composition {
        for s in c.space.iter_mut() {
            let mov0 = mltply_f64_3([inertia_0, inertia_1, inertia_2], TS_F64);
            *s = dd_f32_3(*s, [mov0[0] as f32, mov0[1] as f32, mov0[2] as f32]);
        }
    }
}

pub fn view(anom: &mut Anomaly) -> Vec<Stone> {
    let mut ret: Vec<Stone> = vec![];
    let mut rs: Vec<mpsc::Receiver<Vec<Stone>>> = vec![];

    thread::scope(|s| {
        for mut a in anom.anomaly.iter_mut() {
            let (tx, rx) = mpsc::channel();
            rs.push(rx);
            s.spawn(move || {
                let k = view(&mut a);
                tx.send(k).unwrap();
            });
        }
    });

    for c in anom.component.iter_mut() {
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

    for c in component.component.iter_mut() {
        ret.append(&mut component_view(c));
    }

    let size = component_property(component, MS);

    for c in &component.composition {
        for d in &c.distribution {
            for v in &d(c.space.clone()) {
                let mut s = petrify(magma(2, size as f32));
                move_positions(&mut s.positions, *v);
                ret.push(s);
            }
        }
    }
    ret
}

pub fn add_particle_by(anom: &mut Anomaly, p: Anomaly) {
    anom.anomaly.push(p);
}

pub fn particle(position: [f32; 3], properties: Vec<Property>) -> Anomaly {
    let anom = Anomaly {
        anomaly: vec![],
        component: vec![Component {
            component: vec![],
            composition: vec![Composition {
                space: vec![position],
                distribution: vec![particular],
            }],
            property: properties,
        }],
        force: force_base().force,
    };

    anom
}

//

//

//

// future ref example

pub fn set_inertia(in0: [f64; 3], c: &mut Component) {
    set_component_property(IN0, in0[0], c);
    set_component_property(IN1, in0[1], c);
    set_component_property(IN2, in0[2], c);
}

static EC: f64 = 313.0;
static SP: f64 = 591.0;
static MS: f64 = 343.0;
static CR: f64 = 0.10;
static IN0: f64 = 141.0;
static IN1: f64 = 141.1;
static IN2: f64 = 141.2;
static QMS: [f64; 6] = [2.2, 4.7, 1.28, 96.0, 173.1, 4.18];

pub fn e(position: [f32; 3], inertia: [f64; 3], clock: bool) -> Anomaly {
    let sp = if clock { 0.5 } else { -0.5 };
    particle(
        position,
        vec![
            Property {
                name: SP,
                value: sp,
            },
            Property {
                name: EC,
                value: -1.0,
            },
            Property {
                name: MS,
                value: 0.511,
            },
            Property {
                name: IN0,
                value: inertia[0],
            },
            Property {
                name: IN1,
                value: inertia[1],
            },
            Property {
                name: IN2,
                value: inertia[2],
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
                name: SP,
                value: sp,
            },
            Property {
                name: EC,
                value: ch,
            },
            Property {
                name: MS,
                value: QMS[(flavor % 6) as usize],
            },
            Property {
                name: CR,
                value: (color % 6) as f64,
            },
            Property {
                name: IN0,
                value: inertia[0],
            },
            Property {
                name: IN1,
                value: inertia[1],
            },
            Property {
                name: IN2,
                value: inertia[2],
            },
        ],
    )
}

pub fn particular(coordinates: Vec<[f32; 3]>) -> Vec<[f32; 3]> {
    return coordinates;
}

pub fn force_base() -> Force {
    return Force {
        force: vec![
            // S
            Force {
                force: vec![],
                range: vec![1e-15],
                domain: vec![Component {
                    component: vec![],
                    composition: vec![],
                    property: vec![Property {
                        name: CR,
                        value: 1.0,
                    }],
                }],
            },
            // EM
            Force {
                force: vec![],
                range: vec![f64::MAX],
                domain: vec![Component {
                    component: vec![],
                    composition: vec![],
                    property: vec![Property {
                        name: EC,
                        value: 1.0 / 137.0,
                    }],
                }],
            },
            // W
            Force {
                force: vec![
                    // N
                    Force {
                        force: vec![],
                        range: vec![1e-18],
                        domain: vec![Component {
                            component: vec![],
                            composition: vec![],
                            property: vec![Property {
                                name: MS,
                                value: 1e-13,
                            }],
                        }],
                    },
                    // C
                    Force {
                        force: vec![],
                        range: vec![1e-18],
                        domain: vec![Component {
                            component: vec![],
                            composition: vec![],
                            property: vec![Property {
                                name: SP,
                                value: 1e-13,
                            }],
                        }],
                    },
                ],
                range: vec![],
                domain: vec![],
            },
            // G
            Force {
                force: vec![],
                range: vec![f64::MAX],
                domain: vec![Component {
                    component: vec![],
                    composition: vec![],
                    property: vec![Property {
                        name: MS,
                        value: 1e-41,
                    }],
                }],
            },
        ],
        range: vec![],
        domain: vec![],
    };
}

// pub fn add_particle(anom: &mut Anomaly, position: [f32; 3], properties: Vec<Property>) {
//     anom.anomaly.push(particle(position, properties));
// }
