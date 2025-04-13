#[path = "participant.rs"]
mod participant;
use participant::Participant;
use uuid::Uuid;

pub enum MissionType {
    REGISTER,
    REQUEST_START,
    REQUEST_START,
    REQUEST_START,
    EVENT,
}
impl fmt::Display for MissionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
        // SubjectType::EVENT => write!(f, "event"),
        // SubjectType::REQUEST => write!(f, "request"),
       }
    }
}

pub struct Mission {
    pub mission_id: Uuid,
    pub instance_name: String,
    pub subject_name: Participant,
    pub mission_type: MissionType,
    pub data: String,
}
