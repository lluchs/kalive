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
    pub cancelled: bool,
    pub situations: Vec<SituationRef>,
}

#[derive(Serialize, Default, PartialEq, Debug)]
pub struct SituationRef {
    pub participant_ref: String,
    pub situation_number: String,
}

#[derive(Serialize, Clone, Default, PartialEq, Debug)]
pub struct Situation {
    /// "KVV"
    pub participant_ref: String,
    /// Referenced by SituationFullRef
    pub situation_number: String,
    /// ISO 8601
    pub creation_time: String,
    /// ISO 8601
    pub validity_start_time: String,
    /// ISO 8601
    pub validity_end_time: String,
    pub priority: i32,
    pub scope_type: String,
    /// Title of situation
    pub summary: String,
    /// List of affected lines
    pub description: String,
    /// Description as HTML
    pub detail: String,
}
