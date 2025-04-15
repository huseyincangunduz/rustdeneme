use core::fmt;
use uuid::Uuid;

#[derive(Copy, Clone)]
pub enum SubjectType {
    EVENT, REQUEST
}
impl fmt::Display for SubjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
        SubjectType::EVENT => write!(f, "event"),
        SubjectType::REQUEST => write!(f, "request"),
       }
    }
}

#[derive(Copy, Clone)]
pub struct Participant<'sex> {
    pub client_id: Uuid,
    pub instance_name: &'sex String,
    pub subject_name: &'sex String,
    pub subject_type: SubjectType
}
