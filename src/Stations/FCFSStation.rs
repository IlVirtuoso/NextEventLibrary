use std::collections::VecDeque;

use crate::{Events::Event};

use super::{Station::{IStation, StationEngine}, StationHeader::StationHeader};


pub struct FCFSStation{
      events: VecDeque<Event>,
      engine:StationEngine
}

impl FCFSStation{
      pub const fn new(name:String)->Self{
            FCFSStation { events: VecDeque::new(), engine: StationEngine::new(name) }
      }

      pub fn ProcessArrival(&mut self,evt: Event){

      }

      pub fn ProcessDeparture(&mut self,evt: Event){

      }
}

impl IStation for FCFSStation{
    fn Process(&mut self, event: Event) {
      
    }

    fn Name(&self) -> String {
        self.engine.Name()
    }
}

