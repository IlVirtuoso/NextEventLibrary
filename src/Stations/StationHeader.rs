pub struct StationHeader {
    pub name: String,
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

impl StationHeader {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
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
}
