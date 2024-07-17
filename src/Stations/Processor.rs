use crate::Events;

use super::StationData::StationData;


use log::info;
use serde::{Deserialize, Serialize};

use std::{
    any::Any, collections::{self, VecDeque}, fmt::{format, Display}
};


        //actualClock = self.header.clock;
        //avgInterArrival = self.header.oldclock / self.header.arrivals as f64; /* Average inter-arrival time */
        //avgServiceTime = self.header.busyTime / self.header.completions as f64; /* Average service time */
        //avgDelay = self.header.areaS / self.header.completions as f64; /* Average delay time */
        //avgWaiting = self.header.areaN / self.header.completions as f64; /* Average wait time */
        //utilization = self.header.busyTime / self.header.observationPeriod as f64; /* Utilization */
        //throughput = self.header.completions as f64 / self.header.observationPeriod; /* Throughput */
        //inputRate = self.header.arrivals as f64 / self.header.oldclock as f64; /* Input rate */
        //arrivalRate = self.header.arrivals as f64 / self.header.observationPeriod as f64; /* Arriva rate */
        //serviceRate = self.header.completions as f64 / self.header.busyTime as f64; /* Service rate */
        //traffic = self.header.busyTime / self.header.lastArrival as f64; /* Traffic intensity */
        //meanCustomInQueue = self.header.areaS / self.header.observationPeriod as f64; /* Mean number of customers in queue */
        //meanCustomerInService = self.header.busyTime / self.header.observationPeriod as f64; /* Mean number of customers in service */
        //meanCustomerInSystem = self.header.areaS / self.header.observationPeriod as f64; /* Mean number of customers in system */

use Events::Event;

pub trait IStationProcessor: Sync {
    fn Process(&mut self, event: Event);
}
 

pub struct CoreStationProcessor {
    data: StationData,
}

impl CoreStationProcessor {
    pub const fn new() -> Self {
        Self {
            data: StationData::new(),
        }
    }

    fn ProcessArrival(&mut self, event: &Event) {
        self.data.sysClients += 1;
        self.data.maxClients += if self.data.sysClients > self.data.maxClients {
            1
        } else {
            0
        };
        self.data.arrivals += 1;
        self.data.lastArrival = event.arrivalTime;
    }

    fn ProcessDeparture(&mut self, event: &Event) {
        self.data.sysClients -= 1;
        self.data.completions += 1;
    }



    pub const fn GetHeader(&self) -> StationData {
        self.data
    }
}


impl IStationProcessor for CoreStationProcessor{

     fn Process(&mut self, event: Event) {
        self.data.clock = event.occurTime;
        let interval = self.data.clock - self.data.oldclock;
        self.data.oldclock = event.occurTime;
        if self.data.sysClients > 0 {
            self.data.busyTime += interval;
            self.data.areaN += self.data.sysClients as f64 * interval;
            self.data.areaS += (self.data.sysClients - 1) as f64 * interval;
        }

        match event.kind{
            Events::EventType::ARRIVAL => self.ProcessArrival(&event),
            Events::EventType::DEPARTURE => self.ProcessDeparture(&event),
            _=>{}
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_arrival() {

    }

    #[test]
    fn test_engine_departure(){
    
    }
}