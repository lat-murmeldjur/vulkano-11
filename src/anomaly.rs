pub struct Anomaly {
    pub Anomaly: Vec<Anomaly>,
    pub Component: Vec<Component>,
    pub Force: Vec<Force>,
}

pub struct Composition {
    pub Space: Vec<[f32; 3]>,
    pub Distribution: Vec<fn([f32; 3])>,
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
    pub Range: Vec<f64>,
    pub Domain: Vec<Component>,
}
