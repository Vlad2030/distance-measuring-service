use {
    crate::{
        core,
        models::{location, unit},
    },
    serde,
};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Distance {
    pub points: Vec<location::Location>,
    pub unit: unit::Unit,
    pub distance: Option<f32>,
}

impl Distance {
    pub fn calculate(&self) -> core::result::Result<Self> {
        if self.points.len() < 2 {
            return Err(core::error::Error::Calculate {
                explanation: "location points should be more or equal 2".into(),
            });
        }

        let total_distance: f32 = self
            .points
            .iter()
            .enumerate()
            .map(|(count, location)| {
                let from = location;
                let to = if count + 1 < self.points.len() {
                    &self.points[count + 1]
                } else {
                    return 0.00;
                };

                let d_lat: f32 = (to.latitude - from.latitude).to_radians();
                let d_lon: f32 = (to.longitude - from.longitude).to_radians();

                let lat1: f32 = (from.latitude).to_radians();
                let lat2: f32 = (to.latitude).to_radians();

                let a: f32 = (d_lat / 2.00).sin().powi(2)
                    + (d_lon / 2.00).sin().powi(2) * (lat1.cos() * lat2.cos());

                let c: f32 = 2.00 * a.sqrt().atan2((1.00 - a).sqrt());

                (unit::AVG_EARTH_RADIUS_KM * self.unit.convert()) * c
            })
            .sum();

        Ok(Self {
            points: self.points.clone(),
            unit: self.unit.clone(),
            distance: Some(total_distance),
        })
    }
}

impl Default for Distance {
    fn default() -> Self {
        Self {
            points: [].to_vec(),
            unit: unit::Unit::Kilometers,
            distance: None,
        }
    }
}
