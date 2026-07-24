use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug, Clone)]
struct StationStats {
    min: f64,
    max: f64,
    sum: f64,
    count: u64,
}

impl StationStats {
    fn new(temp: f64) -> Self {
        StationStats {
            min: temp,
            max: temp,
            sum: temp,
            count: 1,
        }
    }

    fn update(&mut self, temp: f64) {
        self.min = self.min.min(temp);
        self.max = self.max.max(temp);
        self.sum += temp;
        self.count += 1;
    }

    fn mean(&self) -> f64 {
        self.sum / self.count as f64
    }
}

fn parse_line(line: &str) -> Option<(&str, f64)> {
    let (station, temp_str) = line.split_once(';')?;
    let temp = temp_str.trim().parse().ok()?;
    Some((station, temp))
}

fn process_file(path: &PathBuf) -> HashMap<String, StationStats> {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut stations: HashMap<String, StationStats> = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        if let Some((station_name, temp)) = parse_line(&line) {
            stations
                .entry(station_name.to_string())
                .and_modify(|stats| stats.update(temp))
                .or_insert_with(|| StationStats::new(temp));
        }
    }

    stations
}

fn print_results(stations: &HashMap<String, StationStats>) {
    println!("{{");

    let mut station_names: Vec<_> = stations.keys().collect();
    station_names.sort();

    for (i, name) in station_names.iter().enumerate() {
        let stats = &stations[*name];
        print!("{}={:.1}/{:.1}/{:.1}", name, stats.min, stats.mean(), stats.max);

        if i < station_names.len() - 1 {
            print!(", ");
        }
    }

    println!();
    println!("}}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        println!("Usage: {} <path_to_measurements.txt>", args[0]);
        println!("Reading from default path: measurements.txt");
        PathBuf::from("measurements.txt")
    };

    println!("Processing file: {}", path.display());

    let stations = process_file(&path);
    println!("Processed {} unique stations", stations.len());

    print_results(&stations);
}
