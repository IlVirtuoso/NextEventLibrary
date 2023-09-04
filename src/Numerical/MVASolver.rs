use super::Matrix::Matrix;



pub struct MVAResult{
    throughputs: Matrix<f64>,
    utilization:Matrix<f64>,
    meanClients: Matrix<f64>,
    meanWaitTime: Matrix<f64>
}




pub fn GeneralMVA(visitRates: Vec<f64>, serviceTimes: Vec<f64>, N: i32) -> MVAResult{
    todo!()
}

