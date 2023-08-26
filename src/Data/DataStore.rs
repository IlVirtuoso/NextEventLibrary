use crate::Stations::Station::StationStatistic;


pub struct DataStore<const BufferSize: usize>{
    dataBuffer: [StationStatistic;BufferSize]
}

impl<const BufferSize: usize> DataStore<BufferSize>{
    pub const fn new(buffer_size: usize) -> Self{
        unsafe{
        DataStore {dataBuffer : Default::default()}
        }
    }
}