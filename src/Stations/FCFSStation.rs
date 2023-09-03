use std::collections::VecDeque;

use crate::{
    Engines::Engine,
    Events::{Event, EventType},
};

use super::{
    Station::{IStation, StationEngine},
    StationHeader::StationHeader,
};

pub struct FCFSStation {
    engine: StationEngine,
    eventQueue: VecDeque<Event>,
    eventUnderProcess: Option<Event>,
}

impl FCFSStation {
    pub const fn new(name: String) -> Self {
        FCFSStation {
            eventQueue: VecDeque::new(),
            engine: StationEngine::new(name),
            eventUnderProcess: None,
        }
    }

    pub fn ProcessArrival(&mut self, evt: Event) {
        if self.eventUnderProcess.is_none() {
            let mut newevt = evt.clone();
            let clock = self.engine.GetHeader().clock;
            self.engine.Process(&evt);
            newevt.arrivalTime = clock;
            newevt.occurTime = clock + newevt.serviceTime;
            newevt.createTime = clock;
            newevt.kind = EventType::DEPARTURE;
            self.eventUnderProcess = Some(newevt.clone());
            Engine::instance().enqueue(newevt);
        } else {
            self.eventQueue.push_back(evt);
        }
    }

    pub fn ProcessDeparture(&mut self, evt: Event) {
        if self.eventUnderProcess.is_none() || *self.eventUnderProcess.as_ref().unwrap() != evt {
            panic!("Event departure requested not in process");
        }
        self.Process(evt.clone());
        if self.engine.GetHeader().sysClients > 0 {
            if self.eventQueue.is_empty() {
                panic!("Event queue should not be empty because clients are not 0");
            }
            let mut new_evt = self.eventQueue.pop_front().unwrap();
            let clock = self.engine.GetHeader().clock;
            new_evt.arrivalTime = clock;
            new_evt.createTime = clock;
            new_evt.occurTime = clock + new_evt.serviceTime;
            new_evt.kind = EventType::DEPARTURE;
            Engine::instance().enqueue(new_evt.clone());
            self.eventUnderProcess = Some(new_evt);
        } else {
            self.eventUnderProcess = None;
        }
    }
}

impl IStation for FCFSStation {
    fn Process(&mut self, event: Event) {
        match event.kind {
            EventType::ARRIVAL => self.ProcessArrival(event),
            EventType::DEPARTURE => self.ProcessDeparture(event),
            EventType::END => (),
            EventType::PROBE => self.engine.Process(&event),
            EventType::MAINTENANCE => (),
            EventType::NOEVENT => (),
        }
    }

    fn Name(&self) -> String {
        self.engine.Name()
    }
}



#[cfg(test)]
mod tests {
    use std::alloc::{alloc, Layout,dealloc};
    use super::*;

    #[test]
    fn test_station_arrival() {

    }
}
