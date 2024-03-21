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
    pub Name: String,
    pub Value: f64,
}

pub struct Force {
    pub Force: Vec<Force>,
    pub Relative: Vec<f64>,
    pub Range: Vec<f64>,
    pub Domain: Vec<Component>,
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
        Force: force().Force,
    };

    anom
}

pub fn particular(coordinates: Vec<[f32; 3]>) -> Vec<[f32; 3]> {
    return coordinates;
}

pub fn force() -> Force {
    return Force {
        Force: vec![
            // S
            Force {
                Force: vec![],
                Relative: vec![1.0],
                Range: vec![],
                Domain: vec![],
            },
            // EM
            Force {
                Force: vec![],
                Relative: vec![1.0 / 137.0],
                Range: vec![f64::MAX],
                Domain: vec![],
            },
            // W
            Force {
                Force: vec![],
                Relative: vec![1e-6],
                Range: vec![],
                Domain: vec![],
            },
            // G
            Force {
                Force: vec![],
                Relative: vec![1e-41],
                Range: vec![f64::MAX],
                Domain: vec![],
            },
        ],
        Relative: vec![],
        Range: vec![],
        Domain: vec![],
    };
}
