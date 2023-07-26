use std::collections::VecDeque;

use crate::{Events::{Event, EventType}, Engines::Engine};

use super::{Station::{IStation, StationEngine}, StationHeader::StationHeader};


pub struct FCFSStation{
      engine:StationEngine,
      eventQueue: VecDeque<Event>,
      eventUnderProcess: Option<Event>
}

impl FCFSStation{
      pub const fn new(name:String)->Self{
            FCFSStation {  eventQueue:VecDeque::new() ,engine: StationEngine::new(name),eventUnderProcess:None }
      }

      pub fn ProcessArrival(&mut self,evt: Event){
            if self.eventUnderProcess.is_none(){
                  let mut newevt = evt.clone();
                  let clock = self.engine.GetHeader().clock;
                  self.engine.Process(evt);
                  newevt.arrivalTime = clock;
                  newevt.occurTime = clock + newevt.serviceTime;
                  newevt.createTime = clock;
                  newevt.kind = EventType::DEPARTURE;
                  self.eventUnderProcess = Some(newevt.clone());
                  Engine::instance().enqueue(newevt);
            }
            else{
                  self.eventQueue.push_back(evt);
            }
      }

      pub fn ProcessDeparture(&mut self,evt: Event){
            self.Process(evt.clone());
            if self.eventUnderProcess.is_none() || *self.eventUnderProcess.as_ref().unwrap() != evt{
                  
            }

      }
}

impl IStation for FCFSStation{
    fn Process(&mut self, event: Event) {
      
    }

    fn Name(&self) -> String {
        self.engine.Name()
    }
}

