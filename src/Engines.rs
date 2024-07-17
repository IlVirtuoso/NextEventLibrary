use std::{
    cell::RefCell,
    collections::{HashMap, LinkedList, VecDeque},
    hash::Hash,
};



use crate::{
    Collections::LightweightList::LwList, Events::Event, Numerical::SystemComposer::StationType, Stations::{Processor::{self, IStationProcessor}, Station::Station}
};

pub struct Engine {
    queue: VecDeque<Event>,
    stations: Vec<Box<Station>>,
}

impl Engine {
    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: Engine = Engine {
            queue: VecDeque::new(),
            stations: Vec::new(),
        };
        unsafe {
            return &mut INSTANCE;
        }
    }

    pub fn enqueue(&mut self, event: Event) {
        let mut iter = (&mut self.queue).into_iter();
        if let Some(i) =  iter.position(|evt| evt.occurTime > event.occurTime){
            self.queue.insert(i, event.clone());
        }
        self.queue.push_back(event);
    }

    pub fn tick(&mut self ) {
        if !self.queue.is_empty() {
            let evt = self.queue.pop_front().unwrap();
            let dest = &evt.destination;

            for station in &mut self.stations {
                if *station.name() == *dest {
                    station.handle(&evt);
                }
            }
            panic!("Not found event with destination {}", dest);
        }
    }

    pub fn register_station(&mut self, station: Box<Station> ) {
        self.stations.push(station);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue() {
        let event = Event::new(
            crate::Events::EventType::ARRIVAL,
            0.0,
            0.0,
            0.0,
            0.0,
            "none".to_string(),
        );

        Engine::instance().enqueue(event);
    }

    #[test]
    fn test_ptr_cast(){
        let event = &mut Event::new(
            crate::Events::EventType::ARRIVAL,
            0.0,
            0.0,
            0.0,
            0.0,
            "none".to_string(),
        ) as *mut Event;

        let mut casted = event as *mut i32;
        unsafe{println!("{:?}", *casted)};
    }
}


