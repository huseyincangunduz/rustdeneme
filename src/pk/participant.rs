use core::fmt;
use uuid::Uuid;

pub enum SubjectType {
    EVENT,
    REQUEST,
}
impl fmt::Display for SubjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SubjectType::EVENT => write!(f, "event"),
            SubjectType::REQUEST => write!(f, "request"),
        }
    }
}

pub struct Participant {
    pub client_id: Uuid,
    pub instance_name: String,
    pub subject_name: String,
    pub subject_type: SubjectType,
}

