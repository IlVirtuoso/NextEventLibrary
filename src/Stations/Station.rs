use super::StationHeader::StationHeader;

use crate::{Events, Data::{DataStore::DataStore, Statistics::StationStatistic}};
use log::info;
use serde::{Deserialize, Serialize};

use std::{collections::{self, VecDeque}, fmt::{Display, format}};
use Events::Event;


pub trait IStation: Sync {
    fn Process(&mut self, event: Event);
    fn Name(&self) -> String;
}

pub struct StationEngine {
    header: StationHeader,
}





impl StationEngine {
    pub const fn new(stationName: String) -> Self {
        Self {
            header: StationHeader::new(stationName),
        }
    }
    pub fn Name(&self) -> String {
        self.header.name.clone()
    }

    fn ProcessArrival(&mut self, event: Event) {
        info!(
            "Processing arrival at {} for event {} at time {}",
            self.header.name, event.kind, event.occurTime
        );
        self.header.sysClients += 1;
        self.header.maxClients += if self.header.sysClients > self.header.maxClients {1} else {0};
        self.header.arrivals += 1;
        self.header.lastArrival = event.arrivalTime;
    }

    fn ProcessDeparture(&mut self, event: Event) {
        info!(
            "Processing departure at {} for event {} ar time {}",
            self.header.name, event.kind, event.occurTime
        );
        self.header.sysClients -= 1;
        self.header.completions += 1;
    }

    pub fn Process(&mut self, event: Event) {
        self.header.clock = event.occurTime;
        info!(
            "Station:{}, with occur time: {}",
            self.header.name, event.occurTime
        );
        let interval = self.header.clock - self.header.oldclock;
        self.header.oldclock = event.occurTime;
        if self.header.sysClients > 0 {
            self.header.busyTime += interval;
            self.header.areaN += self.header.sysClients as f64 * interval;
            self.header.areaS += (self.header.sysClients - 1) as f64 * interval;
        }

        match event.kind {
            Events::EventType::ARRIVAL => self.ProcessArrival(event),
            Events::EventType::DEPARTURE => self.ProcessDeparture(event),
            Events::EventType::PROBE=> DataStore::instance().add_data(self.GetStatistics()),
            _ => {}
        }
    }

    pub fn GetStatistics(&self) -> StationStatistic {
        let mut result: StationStatistic = StationStatistic {
            ..Default::default()
        };
        result.name = self.Name();
        result.actualClock = self.header.clock;
        result.avgInterArrival = self.header.oldclock / self.header.arrivals as f64; /* Average inter-arrival time */
        result.avgServiceTime = self.header.busyTime / self.header.completions as f64; /* Average service time */
        result.avgDelay = self.header.areaS / self.header.completions as f64; /* Average delay time */
        result.avgWaiting = self.header.areaN / self.header.completions as f64; /* Average wait time */
        result.utilization = self.header.busyTime / self.header.observationPeriod as f64; /* Utilization */
        result.throughput = self.header.completions as f64 / self.header.observationPeriod; /* Throughput */
        result.inputRate = self.header.arrivals as f64 / self.header.oldclock as f64; /* Input rate */
        result.arrivalRate = self.header.arrivals as f64 / self.header.observationPeriod as f64; /* Arriva rate */
        result.serviceRate = self.header.completions as f64 / self.header.busyTime as f64; /* Service rate */
        result.traffic = self.header.busyTime / self.header.lastArrival as f64; /* Traffic intensity */
        result.meanCustomInQueue = self.header.areaS / self.header.observationPeriod as f64; /* Mean number of customers in queue */
        result.meanCustomerInService = self.header.busyTime / self.header.observationPeriod as f64; /* Mean number of customers in service */
        result.meanCustomerInSystem = self.header.areaS / self.header.observationPeriod as f64; /* Mean number of customers in system */
        result
    }

    pub const fn GetHeader(&self) -> &StationHeader {
        &self.header
    }
}
