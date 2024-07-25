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
        data.update(event.occurTime);
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
            self.enqueue_event(&newevt);
            self.eventUnderProcess = Some(newevt);
        } else {
            self.eventQueue.push_back(event);
        }
    }

    fn enqueue_event(&mut self, event: &Event){
        if self.engine.is_none() {
            Engine::instance().enqueue(event.clone());
        } else {
            unsafe{
                (*self.engine.unwrap()).enqueue(event.clone());
            }
        }
    }

    pub fn ProcessDeparture(&mut self, evt: &Event, data: &mut StationData) {
        debug_assert!(!(self.eventUnderProcess.is_none()
        || *self.eventUnderProcess.as_ref().unwrap() != *evt
        || evt.subType != DefaultType::INPROCESS),"Event departure requested not in process");
        self.eventUnderProcess = None;
        data.client_departure();
        if data.sysClients > 1 {
            debug_assert!(!self.eventQueue.is_empty(),"Event queue should not be empty while more than 1 client is in the system");
            let mut new_evt = self.eventQueue.pop_front().unwrap();
            let clock = data.clock;
            new_evt.arrivalTime = clock;
            new_evt.createTime = clock;
            new_evt.occurTime = clock + new_evt.serviceTime;
            new_evt.kind = DefaultType::DEPARTURE.into();
            self.enqueue_event(&new_evt);
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
    

    #[test]
    fn test_station_arrival() {
        let mut engine = Engine::new();
        let mut station = Station::new("Mock");
        let mut handler = FCFSPolicyManager::new();
        handler.use_engine(&mut engine as *mut Engine);
        station.set_handler(Box::new(handler));
        engine.register_station(Box::new(station));
        for i in 0..100 {
            let mut event = Event::gen_arrival(engine.stations()[0].get_data().clock + Exponential(10.0));
            event.destination = engine.stations()[0].name().clone();
            engine.enqueue(event);
            while engine.has_events() {
                engine.tick();
            }
        }

        println!("{:?}",engine.stations()[0].get_data());
    }
}
