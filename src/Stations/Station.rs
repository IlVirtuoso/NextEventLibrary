

use std::{any::Any, borrow::Borrow};

use crate::Events::Event;

use super::{Processor::{CoreStationProcessor, IStationProcessor}, StationData::StationData};

type Processor = Box<dyn IStationProcessor>;

pub struct Station{
    name: String,
    processors: Vec<Processor>,
    arrivalProcessor: Option<Processor>,
    departureProcessor: Option<Processor>
}



impl Station {
    fn new(name: &str) -> Self {
        Station {
            name: name.to_string(),
            processors: vec![],
            arrivalProcessor: None,
            departureProcessor: None
        }
    }

    fn process(&mut self,event : Event){
        for processor in &mut self.processors{
            processor.Process(event);
        }
    }

    fn add_processor(&mut self, processor: Box<dyn IStationProcessor>){
        self.processors.push(processor);
    }



    fn get_data(&self) -> StationData{
       
    }

    fn name(&self) -> &String {
        &self.name
    }
}