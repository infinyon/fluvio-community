

use fluvio_smartmodule::{smartmodule, Result, Record, RecordData};
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize,Serialize)]
pub struct Speed {
    lat: f32,
    long: f32,
    route: String,
    speed: f32,
    vehicle: u32
}

#[derive(Default, Deserialize,Serialize)]
pub struct Accumulator {
    total: f32,
    average: f32,
    last: f32,
    count: u32
}

impl Accumulator {
    fn add_speed(&mut self, speed: f32) {
        self.total += speed;
        self.count += 1;
        self.last = speed;
        self.average = self.total / self.count as f32;
    }
}



#[smartmodule(aggregate)]
pub fn aggregate(accumulator: RecordData, current: &Record) -> Result<RecordData> {

    let speed: Speed = serde_json::from_slice(current.value.as_ref())?;
    let mut accumulator: Accumulator = match serde_json::from_slice(accumulator.as_ref()) {
        Ok(acc) => acc,
        Err(_) => Accumulator::default()
    };

    accumulator.add_speed(speed.speed);

    let output_json = serde_json::to_vec(&accumulator)?;
    Ok(output_json.into())
}



