use std::{
    borrow::BorrowMut, cell::RefCell, collections::{HashMap, LinkedList, VecDeque}, hash::Hash
};

use crate::{Events::Event, Stations::Station::{IEventManager, Station}};





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

    pub fn new()-> Self{
        Engine { queue: VecDeque::new(), stations: Vec::new() }
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
                    station.as_mut().handle(&evt);
                    return;
                }
            }
            panic!("Not found event with destination {}", dest);
        }
    }

    pub fn stations(&self)-> &Vec<Box<Station>>{
        &self.stations
    }

    pub fn register_station(&mut self, station: Box<Station> ) {
        self.stations.push(station);
    }

    pub fn has_events(&self)->bool{
        !self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::Events::DefaultType;

    use super::*;

    #[test]
    fn test_enqueue() {
        let event = Event::new(
            DefaultType::ARRIVAL.into(),
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
            DefaultType::ARRIVAL.into(),
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


