// experimental generic composite reductive approximation outline
use std::thread;

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

static EC: f64 = 313.0;
static Sp: f64 = 591.0;
static Ms: f64 = 343.0;
static Cr: f64 = 0.10;
static QMs: [f64; 6] = [2.2, 4.7, 1.28, 96.0, 173.1, 4.18];

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

pub fn e(position: [f32; 3], clock: bool) {
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
    );
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
