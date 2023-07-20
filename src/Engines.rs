use std::{
    collections::{HashMap, LinkedList, VecDeque},
    hash::Hash,
};

use crate::{
    Events::Event,
    Stations::Station::{self, IStation},
};

pub struct Engine {
    queue: LinkedList<(String, Event)>,
    stations: Vec<&'static dyn IStation>,
}

impl Engine {
    fn instance() -> &'static mut Self {
        static mut INSTANCE: Engine = Engine {
            queue: LinkedList::new(),
            stations: Vec::new(),
        };
        unsafe {
            return &mut INSTANCE;
        }
    }

    pub fn enqueue(&mut self,event: Event, stationName: String) {
    }

    pub fn tick(&mut self){
      
    }
}
