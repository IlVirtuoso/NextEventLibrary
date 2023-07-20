pub struct StationHeader {
    name: String,
    _arrivals: i32,
    _completions: i32,
    _sysClients: i32,
    _maxClients: i32,
    _busyTime: f64,
    _observationPeriod: f64,
    _lastArrival: f64,
    _areaN: f64,
    _areaS: f64,
    _oldclock: f64,
    _clock: f64,
}

impl StationHeader {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            _arrivals: 0,
            _completions: 0,
            _sysClients: 0,
            _maxClients: 0,
            _busyTime: 0.0,
            _observationPeriod: 0.0,
            _lastArrival: 0.0,
            _areaN: 0.0,
            _areaS: 0.0,
            _oldclock: 0.0,
            _clock: 0.0,
        }
    }
}
