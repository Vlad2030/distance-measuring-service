use serde;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Location {
    pub latitude: f32,
    pub longitude: f32,
}
