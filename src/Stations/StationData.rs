use crate::Events::Event;

#[derive(Clone, Copy,Debug)]
pub struct StationData {
    pub arrivals: i32,
    pub completions: i32,
    pub sysClients: i32,
    pub maxClients: i32,
    pub busyTime: f64,
    pub observationPeriod: f64,
    pub lastArrival: f64,
    pub areaN: f64,
    pub areaS: f64,
    pub oldclock: f64,
    pub clock: f64,
}

impl StationData {
    pub const fn new() -> Self {
        Self {
            arrivals: 0,
            completions: 0,
            sysClients: 0,
            maxClients: 0,
            busyTime: 0.0,
            observationPeriod: 0.0,
            lastArrival: 0.0,
            areaN: 0.0,
            areaS: 0.0,
            oldclock: 0.0,
            clock: 0.0,
        }
    }


    pub fn update(&mut self, occurTime: f64){
        self.clock = occurTime;
        let interval = self.clock - self.oldclock;
        self.oldclock = occurTime;
        if self.sysClients > 0 {
            self.busyTime += interval;
            self.areaN += self.sysClients as f64 * interval;
            self.areaS += (self.sysClients - 1) as f64 * interval;
        }
    }

    pub fn client_arrived(&mut self, arrivalTime: f64){
        self.sysClients += 1;
        self.maxClients += if self.sysClients > self.maxClients {
            1
        } else {
            0
        };
        self.arrivals += 1;
        self.lastArrival = arrivalTime;
    }

    pub fn client_departure(&mut self){
        self.sysClients -= 1;
        self.completions += 1;
    }
}


