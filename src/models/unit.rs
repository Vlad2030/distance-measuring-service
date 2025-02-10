use serde;

pub const AVG_EARTH_RADIUS_KM: f32 = 6_371.009;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum Unit {
    Kilometers,
    Miles,
}

impl Unit {
    pub fn convert(&self) -> f32 {
        match self {
            Self::Kilometers => 1.00,
            Self::Miles => 0.621_371_2,
        }
    }

    pub fn as_vec() -> Vec<Self> {
        vec![Self::Kilometers, Self::Miles]
    }
}
