use std::convert::TryFrom;

pub enum Service {
    PrintInt = 1
}

impl TryFrom<i32> for Service {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Service::PrintInt),
            _ => Err("out of bounds")
        }
    }
}