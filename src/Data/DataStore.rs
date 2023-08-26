use crate::{
    Data,
    Stations::Station::{self, StationStatistic},
};
use std::{
    fs::File,
    path::{Path, PathBuf}, error::Error, io::Write,
};

pub(crate) static mut DATA_FILE_NAME: &'static str = "data.csv";
pub(crate) static mut DATA_STORE: DataStore = DataStore::new();

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
    dataBuffer: [Option<StationStatistic>; buffer_size()],
    alloc: usize,
}

impl DataStore {
    const fn new() -> Self {
        const DEFAULT_VALUE: Option<StationStatistic> = None;
        DataStore {
            dataBuffer: [DEFAULT_VALUE; buffer_size()],
            alloc: 0,
        }
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
        result.push_str(&StationStatistic::get_header(';'));
        for data in self.dataBuffer.as_ref(){
            let fmt = data.as_ref().unwrap().to_format(';');
            result.push_str(&fmt);
        }
        data_file.write_all(result.as_bytes()).expect("Failed to write data in file");
    }

    pub fn add_data(&mut self, data: StationStatistic) {
        if self.alloc >= self.dataBuffer.len() {
            self.flush_to_file();
        }
        self.dataBuffer[self.alloc] = Some(data);
        self.alloc += 1;
    }
}
