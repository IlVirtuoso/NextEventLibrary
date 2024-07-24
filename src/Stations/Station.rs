
use log::error;
use plotters::prelude::DynElement;

use crate::Events::{DefaultType, Event};

use super::{
    StationData::StationData,
};


pub trait IEventManager {
    fn process_event(
        &mut self,
        event: &Event,
        data: &mut StationData,
    );
}


type Handler = Option<Box<dyn IEventManager>>;

pub struct Station {
    name: String,
    data: StationData,
    event_handler: Handler,
    arrival_handler: Handler,
    departure_handler: Handler
}

impl Station {
    pub fn new(name: &str) -> Self {
        let mut s = Station {
            name: name.to_string(),
            data: StationData::new(),
            event_handler: None,
            arrival_handler: None,
            departure_handler: None,
        };
        s
    }

    pub fn set_handler(&mut self, handler: Box<dyn IEventManager>) {
        if let None = self.event_handler {
            self.event_handler = Some(handler);
        } else {
            error!("Cannot set handler 2 times")
        }
    }

    pub fn handle(&mut self, event: &Event) {
        if let Some(handler) = &mut self.event_handler {
            self.data.update(event.occurTime);
            if event.kind == DefaultType::ARRIVAL && self.arrival_handler.is_some(){
                self.arrival_handler.as_mut().unwrap().process_event(event, &mut self.data);
            }
            else if event.kind == DefaultType::DEPARTURE && self.departure_handler.is_some(){
                self.departure_handler.as_mut().unwrap().process_event(event, &mut self.data);
            }
            handler.process_event(event, &mut self.data);
        } else {
            error!("handler not set");
        }
    }

    pub fn get_data(&self) -> &StationData {
        &self.data
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;

    use crate::Stations::FCFSRuler::FCFSPolicyManager;

    use super::*;

    #[test]
    fn test_name_equality(){

    }
    
    #[test]
    fn test_forwarder() {
        let mut station = Station::new("mock");
        station.set_handler(Box::new(FCFSPolicyManager::new()));
        station.handle(&Event::gen_arrival(10.0));
        println!("{:?}", station.get_data())
    }
}
