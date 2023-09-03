use serde::{Serialize, Deserialize};

use crate::Random::{rngs::RandomGenerator, rvgs::{Normal, Poisson, Exponential}};

#[derive(Default,Clone,Serialize,Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StationStatistic {
    pub name: String,
    pub actualClock: f64,
    pub avgInterArrival: f64,
    pub avgServiceTime: f64,
    pub avgDelay: f64,
    pub avgWaiting: f64,
    pub utilization: f64,
    pub throughput: f64,
    pub inputRate: f64,
    pub arrivalRate: f64,
    pub serviceRate: f64,
    pub traffic: f64,
    pub meanCustomInQueue: f64,
    pub meanCustomerInService: f64,
    pub meanCustomerInSystem: f64,
}

impl StationStatistic{
    pub fn Random()-> Self{
        let mut result = StationStatistic::default();
        result.actualClock = Normal(10.0, 20.0);
        result.arrivalRate = Normal(5.0, 10.0);
        result.avgDelay = Exponential(10.0);
        result.avgInterArrival= Exponential(5.0);
        result.avgServiceTime = Exponential(10.0);
        result.avgWaiting = Exponential(5.0);
        result.inputRate = Exponential(4.0);
        result.inputRate = Exponential(3.0);
        result.meanCustomInQueue = Poisson(3.0) as f64;
        result
    }
}