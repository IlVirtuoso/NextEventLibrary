use once_cell::sync::Lazy;


use std::{
    fs::File,
    path::{Path, PathBuf}, error::Error, io::Write,
};

use super::Statistics::StationStatistic;

pub(crate) static mut DATA_FILE_NAME: &'static str = "data.csv";



const fn parse_u32(s: &str) -> u32 {
    let mut out: u32 = 0;
    let mut i: usize = 0;
    while i < s.len() {
        out *= 10;
        out += (s.as_bytes()[i] - b'0') as u32;
        i += 1;
    }
    out
}

const fn buffer_size() -> usize {
    match option_env!("BUFFER_SIZE") {
        Some(t) => parse_u32(t) as usize,
        None => 1000,
    }
}

pub struct DataStore {
    dataBuffer: Vec<StationStatistic>,
    alloc: usize,
}

impl DataStore {
    fn new() -> Self {
        DataStore {
            dataBuffer: vec![StationStatistic::default();buffer_size()],
            alloc: 0,
        }
    }

    pub(crate) fn instance()-> &'static mut DataStore{
        static mut INSTANCE : Lazy<DataStore> = Lazy::new(|| DataStore::new());
        unsafe{&mut INSTANCE}
    }

    pub fn flush_to_file(&mut self) {
        let mut data_file = {
            unsafe {
                if Path::new(DATA_FILE_NAME).exists() {
                    File::open(DATA_FILE_NAME).expect("Error in opening file")
                } else {
                    File::create(DATA_FILE_NAME).expect("Error in creating file")
                }
            }
        };

        let mut result = String::new();
        let mut writer  = csv::Writer::from_writer(data_file);

        for data in &mut self.dataBuffer{
            writer.serialize(data).expect("Failed to write record");
        }
        writer.flush().expect("failed to flush");
        
        self.alloc = 0;
    }

    pub fn add_data(&mut self, data: StationStatistic) {
        if self.alloc >= self.dataBuffer.len() {
            self.flush_to_file();
        }
        self.dataBuffer[self.alloc] = data;
        self.alloc += 1;
    }
}


#[cfg(test)]
mod tests {
    use crate::Data;

    use super::*;

    #[test]
    fn test_store() {
        
    }
}