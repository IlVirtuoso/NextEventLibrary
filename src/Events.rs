use core::fmt;

use NESLib_macros::LwItem;

use crate::Collections::LightweightList::ILwItem;
use crate::Collections::LightweightList::LwHeader;
use crate::Random::rvgs::Exponential;

#[derive(Clone, Copy, PartialEq)]
pub enum EventType {
    ARRIVAL,
    DEPARTURE,
    END,
    PROBE,
    MAINTENANCE,
    NOEVENT,
}

#[derive(Clone, PartialEq)]
pub struct Event {
    pub kind: EventType,
    pub createTime: f64,
    pub occurTime: f64,
    pub serviceTime: f64,
    pub arrivalTime: f64,
    pub subType: EventType,
    pub destination: String,
}

impl Event {
    pub fn new(
        kind: EventType,
        createTime: f64,
        occurTime: f64,
        serviceTime: f64,
        arrivalTime: f64,
        destination: String,
    ) -> Self {
        Event {
            kind,
            createTime,
            occurTime,
            serviceTime,
            arrivalTime,
            subType: EventType::NOEVENT,
            destination,
        }
    }

    pub fn gen_arrival(clock: f64) -> Self {
        Event {
            kind: EventType::ARRIVAL,
            createTime: clock ,
            occurTime: clock + Exponential(3.0),
            serviceTime: Exponential(5.0),
            arrivalTime: clock + Exponential(1.0),
            subType: EventType::ARRIVAL,
            destination: "None".to_string(),
        }
    }
    pub fn gen_departure(clock: f64) -> Self{
        Event{
            kind: EventType::DEPARTURE,
            createTime: clock,
            occurTime: clock + Exponential(3.0),
            serviceTime: 0.0,
            arrivalTime: clock - Exponential(3.0),
            subType: EventType::NOEVENT,
            destination: "None".to_string()
        }
    }
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: &str = "";
        match self {
            EventType::ARRIVAL => result = "Arrival",
            EventType::DEPARTURE => result = "Departure",
            EventType::END => result = "End",
            EventType::PROBE => result = "Probe",
            EventType::MAINTENANCE => result = "Maintenance",
            EventType::NOEVENT => result = "NoEvent",
        }
        f.write_str(result)
    }
}
