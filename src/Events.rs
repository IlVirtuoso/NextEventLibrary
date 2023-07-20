
#[derive(Clone,Copy)]
pub enum EventType{
      ARRIVAL,
      DEPARTURE,
      END,
      PROBE,
      MAINTENANCE,
      NOEVENT
}

#[derive(Clone,Copy)]
pub struct Event{
      pub kind : EventType,
      pub createTime: f64,
      pub occurTime: f64,
      pub serviceTime: f64,
      pub arrivalTime: f64,
      pub subType: EventType
}

impl Event{
      pub fn new(kind: EventType,createTime: f64, occurTime: f64, serviceTime:f64, arrivalTime:f64) -> Self{
            Event { kind, createTime, occurTime, serviceTime, arrivalTime, subType: EventType::NOEVENT }
      }      
}

