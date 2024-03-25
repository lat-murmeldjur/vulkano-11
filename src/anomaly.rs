// experimental generic composite reductive approximation outline
use std::thread;

use crate::magma_ocean::{magma, petrify, Stone};
use crate::positions::{move_positions, Normal, Position};

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

#[derive(Clone)]
pub struct Property {
    pub Name: f64,
    pub Value: f64,
}

pub struct Force {
    pub Force: Vec<Force>,
    pub Range: Vec<f64>,
    pub Domain: Vec<Component>,
}

pub fn add_particle(anom: &mut Anomaly, position: [f32; 3], properties: Vec<Property>) {
    anom.Anomaly.push(particle(position, properties));
}

pub fn add_particle_by(anom: &mut Anomaly, p: Anomaly) {
    anom.Anomaly.push(p);
}

pub fn particle(position: [f32; 3], properties: Vec<Property>) -> Anomaly {
    let mut anom = Anomaly {
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

pub fn view(anom: &Anomaly) -> Vec<Stone> {
    let mut ret: Vec<Stone> = vec![];
    for a in &anom.Anomaly {
        ret.append(&mut view(&a));
    }

    for c in &anom.Component {
        ret.append(&mut component_view(&c));
    }

    ret
}

pub fn component_view(component: &Component) -> Vec<Stone> {
    let mut ret: Vec<Stone> = vec![];

    for c in &component.Component {
        ret.append(&mut component_view(&c));
    }

    let size: Vec<Property> = component
        .Property
        .clone()
        .into_iter()
        .filter(|c| c.Name == Ms)
        .collect();

    for c in &component.Composition {
        for d in &c.Distribution {
            for v in &d(c.Space.clone()) {
                let mut s = petrify(magma(2, size[0].Value as f32));
                move_positions(&mut s.positions, *v);
                ret.push(s);
            }
        }
    }
    ret
}

//

//

//

// future ref example

static EC: f64 = 313.0;
static Sp: f64 = 591.0;
static Ms: f64 = 343.0;
static Cr: f64 = 0.10;
static QMs: [f64; 6] = [2.2, 4.7, 1.28, 96.0, 173.1, 4.18];

pub fn e(position: [f32; 3], clock: bool) -> Anomaly {
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
        ],
    )
}

pub fn q(position: [f32; 3], clock: bool, charge: bool, color: u8, flavor: u8) {
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
        ],
    );
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
