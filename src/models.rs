use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Walk {
    pub walk: Vec<f64>,
}

#[derive(Debug)]
pub struct RandomWalks {
    pub walks: Vec<Walk>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationResults {
    pub fifth: Walk,
    pub fiftieth: Walk,
    pub ninety_fifth: Walk,
}

impl Walk {
    pub fn new(walk: Vec<f64>) -> Self {
        Self { walk }
    }
}

impl RandomWalks {
    pub fn new(walks: Vec<Walk>) -> Self {
        Self { walks }
    }
}

impl SimulationResults {
    pub fn new(fifth: Walk, fiftieth: Walk, ninety_fifth: Walk) -> Self {
        Self {
            fifth,
            fiftieth,
            ninety_fifth,
        }
    }
}
