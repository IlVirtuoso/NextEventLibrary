use std::{
    collections::{HashMap, LinkedList, VecDeque},
    hash::Hash, cell::RefCell,
};

use crate::{
    Events::Event,
    Stations::Station::{self, IStation}, Collections::LightweightList::LwList,
};


pub struct Engine {
    queue: LwList<Event>,
    stations: Vec<&'static dyn IStation>,
}

impl Engine {
    fn instance() -> &'static mut Self {
        static mut INSTANCE: Engine = Engine {
            queue: LwList::new(),
            stations: Vec::new(),
        };
        unsafe {
            return &mut INSTANCE;
        }
    }

    pub fn enqueue(&mut self,event: Event) {
        self.queue.push(Box::into_raw(Box::new(event)));
    }

    pub fn tick(&mut self){
      
    }
}
