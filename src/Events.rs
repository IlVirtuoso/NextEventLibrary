use core::fmt;

use NESLib_macro::LwItem;

use crate::Collections::LightweightList::ILwItem;
use crate::Collections::LightweightList::LwHeader;

#[derive(Clone, Copy)]
pub enum EventType {
    ARRIVAL,
    DEPARTURE,
    END,
    PROBE,
    MAINTENANCE,
    NOEVENT,
}

#[derive(LwItem)]
pub struct Event {
   #[Header] _header: LwHeader,
    pub kind: EventType,
    pub createTime: f64,
    pub occurTime: f64,
    pub serviceTime: f64,
    pub arrivalTime: f64,
    pub subType: EventType,
}

impl Event {
    pub fn new(
        kind: EventType,
        createTime: f64,
        occurTime: f64,
        serviceTime: f64,
        arrivalTime: f64,
    ) -> Self {
        Event {
            _header: LwHeader::new(),
            kind,
            createTime,
            occurTime,
            serviceTime,
            arrivalTime,
            subType: EventType::NOEVENT,
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
