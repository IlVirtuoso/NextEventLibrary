use std::{borrow::Borrow, collections::VecDeque, rc::Rc};

use crate::{
    Engines::Engine,
    Events::{DefaultType, Event},
};

use super::{Station::IEventManager, StationData::StationData};

pub struct FCFSPolicyManager {
    eventQueue: VecDeque<Event>,
    eventUnderProcess: Option<Event>,
    engine: Option<*mut Engine>,
}

impl IEventManager for FCFSPolicyManager {
    fn process_event(&mut self, event: &Event, data: &mut super::StationData::StationData) {
        match DefaultType::from(event.kind) {
            DefaultType::ARRIVAL => self.ProcessArrival(event, data),
            DefaultType::DEPARTURE => self.ProcessDeparture(event, data),
            _ => {}
        }
    }
}

impl FCFSPolicyManager {
    pub fn new() -> Self {
        FCFSPolicyManager {
            eventQueue: VecDeque::new(),
            eventUnderProcess: None,
            engine: None,
        }
    }

    pub fn use_engine(&mut self, engine: *mut Engine) {
        self.engine = Some(engine);
    }

    pub fn ProcessArrival(&mut self, evt: &Event, data: &mut StationData) {
        let mut event = evt.clone();
        if evt.subType != DefaultType::INPROCESS {
            data.client_arrived(evt.arrivalTime);
            event.subType = DefaultType::INPROCESS.into();
        }
        if self.eventUnderProcess.is_none() {
            let mut newevt = event;
            let clock = data.clock;
            newevt.arrivalTime = clock;
            newevt.occurTime = clock + newevt.serviceTime;
            newevt.createTime = clock;
            newevt.kind = DefaultType::DEPARTURE.into();
            self.eventUnderProcess = Some(newevt.clone());
            Engine::instance().enqueue(newevt);
        } else {
            self.eventQueue.push_back(event);
        }
    }

    pub fn ProcessDeparture(&mut self, evt: &Event, data: &mut StationData) {
        if self.eventUnderProcess.is_none()
            || *self.eventUnderProcess.as_ref().unwrap() != *evt
            || evt.subType != DefaultType::INPROCESS
        {
            panic!("Event departure requested not in process");
        }
        self.eventUnderProcess = None;

        if data.sysClients > 0 {
            if self.eventQueue.is_empty() {
                panic!("Event queue should not be empty because clients are not 0");
            }
            let mut new_evt = self.eventQueue.pop_front().unwrap();
            data.client_departure();
            let clock = data.clock;
            new_evt.arrivalTime = clock;
            new_evt.createTime = clock;
            new_evt.occurTime = clock + new_evt.serviceTime;
            new_evt.kind = DefaultType::DEPARTURE.into();
            if self.engine.is_none() {
                Engine::instance().enqueue(new_evt.clone());
            } else {
                unsafe{
                    (*self.engine.unwrap()).enqueue(new_evt.clone());
                }
            }
            self.eventUnderProcess = Some(new_evt);
        } else {
            self.eventUnderProcess = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Random::{rngs::RandomGenerator, rvgs::Exponential},
        Stations::Station::Station,
    };

    use super::*;
    use std::alloc::{alloc, dealloc, Layout};

    #[test]
    fn test_station_arrival() {
        let mut data = StationData::new();
        let mut engine = Engine::new();
        let mut station = Station::new("Mock");
        station.set_handler(Box::new(FCFSPolicyManager::new()));
        for i in 0..100 {
            let mut event = Event::gen_arrival(data.clock + Exponential(10.0));
            event.destination = station.name().clone();
            
        }
    }
}
