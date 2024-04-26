use serde_derive::Serialize;

pub mod trias;

#[derive(Serialize, Default, PartialEq, Debug)]
pub struct Stop {
    pub id: String,
    pub name: String,
    pub lat: f32,
    pub long: f32,
    pub modes: Vec<String>,
}

#[derive(Serialize, Default, PartialEq, Debug)]
pub struct Departure {
    pub line: String,
    pub destination: String,
    pub bay: Option<String>,
    pub mode: String,
    pub mode_name: String,
    pub timetable_time: String,
    pub estimated_time: Option<String>,
}
