use scrypto::prelude::*;

const ARRAY_LENGTH: u16 = 4;

#[derive(ScryptoCategorize, LegacyDescribe, ScryptoEncode, ScryptoDecode, Clone)]
pub struct Observation {
    timestamp: i64,
    step: u16
}

impl Observation{

    pub fn from(timestamp: i64, step:u16) -> Self
    {
        Self {
            timestamp,
            step
        }
    }
}

#[derive(ScryptoCategorize, LegacyDescribe, ScryptoEncode, ScryptoDecode, Clone)]
pub struct ObservationArray {
    start: u16,
    data: Vec<Observation>,
}

impl ObservationArray {

    pub fn new() -> Self {
        Self {
            start: 0,
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, timestamp: i64, step: u16) {

        let new_obs = Observation::from(timestamp, step);

        if self.data.len() < ARRAY_LENGTH as usize {
            self.data.push(new_obs);
        }
        else {
            self.data[self.start as usize] = new_obs;
            self.start+=1;
        }
    }

    pub fn get(&self, index: u16) -> Option<&Observation> {
        if index as usize >= self.data.len()
        {
            return None;
        }
        let index = ((index + self.start) as usize) % (ARRAY_LENGTH as usize);
        self.data.get(index)
    }

    pub fn get_time_weighted_average_step_since(&self, timestamp: i64, current_timestamp: i64) -> u16
    {
        // Get index of the first observation made at timestamp
        let start_index = self.get_first_observation_index_since(timestamp);
        let mut total = 0;
        let stop_index = if self.data.len() < ARRAY_LENGTH as usize {
            self.data.len()
        }
        else
        {
            self.start as usize + ARRAY_LENGTH as usize
        };

        for i in start_index..stop_index-1 {
            let true_i = i % (ARRAY_LENGTH as usize);
            let next_true_i = (i+1) % (ARRAY_LENGTH as usize);
            let obs_1 = self.data.get(true_i).unwrap();
            let obs_2 = self.data.get(next_true_i).unwrap();
            total +=  (obs_2.timestamp - obs_1.timestamp)* (obs_1.step as i64);
        }
        let last_obs = self.data.get((stop_index - 1) %(ARRAY_LENGTH as usize) ).unwrap();
        total+= (current_timestamp - last_obs.timestamp)*(last_obs.step as i64);
        (total / (current_timestamp - timestamp)) as u16
    }

    fn get_first_observation_index_since(&self, timestamp: i64) -> usize
    {
        let mut start_index = self.start as usize;

        if timestamp < self.data[start_index].timestamp
        {
            return start_index
        }

        let mut stop_index: usize = self.start as usize + self.data.len();
        loop {
            if start_index == stop_index { break; };
            let check_index_raw = stop_index - start_index;
            let check_index = check_index_raw % (ARRAY_LENGTH as usize);
            let found_timestamp = self.data.get(check_index).unwrap().timestamp;
            if found_timestamp > timestamp
            {
                stop_index = check_index_raw;
            }
            else if found_timestamp < timestamp
            {
                start_index = check_index_raw;
            }
            else
            {
                break;
            }
        }

        stop_index
    }
}