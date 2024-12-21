// Preparation: download dataset to local filesystem with
// wget https://github.com/gunnarmorling/1brc/blob/main/data/weather_stations.csv

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Debug)]
struct TempStat {
    min: f32,
    max: f32,
    total: f32,
    count: usize,
}
impl TempStat {
    /// Initialize a new temporature stat.
    fn new(temp: f32) -> Self {
        Self {
            min: temp,
            max: temp,
            total: temp,
            count: 1,
        }
    }

    /// Add a new temporature data point to the current stat.
    fn add_new_temp(&mut self, temp: f32) {
        self.min = f32::min(self.min, temp);
        self.max = f32::max(self.max, temp);
        self.total += temp;
        self.count += 1;
    }

    /// Get the average temporature.
    fn aver_temp(&self) -> f32 {
        self.total / self.count as f32
    }
}

fn main() -> std::io::Result<()> {
    let start_time = Instant::now(); // Used to calculation time elapse.

    let file_path = "/home/ubuntu/1bc-rs/src/weather_station.csv";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Maps from city to temporature stats.
    let mut map: HashMap<String, TempStat> = HashMap::new();

    for line in reader.lines() {
        // `collect` transforms an iterator into a container.
        let line = line.unwrap();
        let mut parts = line.split(';');
        let city = parts.next().expect("Fails to parse city name.").to_owned();
        let temp = parts
            .next()
            .expect("Fails to parse temporature")
            .parse::<f32>()
            .expect("Fails to temporature convert to f32.");

        map.entry(city)
            .and_modify(|cur_stat: &mut TempStat| cur_stat.add_new_temp(temp))
            .or_insert(TempStat::new(temp));
    }

    // Print out timing result.
    let duration = start_time.elapsed();
    println!("Time elapsed for the code segment: {:?}", duration);

    // Check aggregated results.
    // for (cur_city, cur_stat) in map.iter() {
    //     println!("City: {}, min: {:.2}, max: {:.2}, avg: {:.2}", cur_city, cur_stat.min, cur_stat.max, cur_stat.aver_temp())
    // }

    Ok(())
}
