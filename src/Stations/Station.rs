use crate::Events;
use Events::Event;
use std::collections::{self, VecDeque};
use super::StationHeader::StationHeader;

pub trait IStation : Sync {
      fn Process(& mut self,event: Event);
      fn Name(&self) -> String;
}


pub struct StationEngine{
      header: StationHeader,
      queue: VecDeque<Event>
}





