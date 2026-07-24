use rayon::prelude::*;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, Clone)]
struct StationStats {
    min_temp: f64,
    max_temp: f64,
    sum_temp: f64,
    count: u64,
}

impl StationStats {
    fn new(temperature: f64) -> Self {
        StationStats {
            min_temp: temperature,
            max_temp: temperature,
            sum_temp: temperature,
            count: 1,
        }
    }

    fn update(&mut self, temperature: f64) {
        self.min_temp = self.min_temp.min(temperature);
        self.max_temp = self.max_temp.max(temperature);
        self.sum_temp += temperature;
        self.count += 1;
    }

    fn mean(&self) -> f64 {
        self.sum_temp / self.count as f64
    }

    // Merge two StationStats (for combining results from different threads)
    fn merge(&mut self, other: &StationStats) {
        self.min_temp = self.min_temp.min(other.min_temp);
        self.max_temp = self.max_temp.max(other.max_temp);
        self.sum_temp += other.sum_temp;
        self.count += other.count;
    }
}

fn parse_measurement_line(line: &str) -> Option<(&str, f64)> {
    // Skip comment lines
    if line.starts_with('#') || line.is_empty() {
        return None;
    }

    // Parse 1BRC format: station_name;temperature
    // Example: "Tokyo;35.6897" or "New York;-12.5"
    let parts: Vec<&str> = line.split(';').collect();
    if parts.len() == 2 {
        let station_name = parts[0].trim();
        let temp_str = parts[1].trim();

        // Parse temperature, skip if invalid
        if let Ok(temperature) = temp_str.parse::<f64>() {
            return Some((station_name, temperature));
        }
    }
    None
}

fn process_chunk(lines: Vec<String>) -> HashMap<String, StationStats> {
    let mut stations: HashMap<String, StationStats> = HashMap::new();

    for line in lines {
        if let Some((station_name, temperature)) = parse_measurement_line(&line) {
            stations
                .entry(station_name.to_string())
                .and_modify(|stats| stats.update(temperature))
                .or_insert_with(|| StationStats::new(temperature));
        }
    }

    stations
}

fn process_file_parallel(path: &PathBuf) -> HashMap<String, StationStats> {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    // Read all lines into a vector (for large files, you'd use chunked reading)
    let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

    println!("Read {} lines from file", lines.len());

    // Split into chunks for parallel processing
    let num_threads = rayon::current_num_threads();
    let chunk_size = (lines.len() + num_threads - 1) / num_threads;

    let chunks: Vec<_> = lines
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();

    println!(
        "Processing {} chunks using {} threads",
        chunks.len(),
        num_threads
    );

    // Process chunks in parallel
    let results: Vec<HashMap<String, StationStats>> =
        chunks.into_par_iter().map(process_chunk).collect();

    // Merge results from all threads
    let mut merged_stations: HashMap<String, StationStats> = HashMap::new();

    for thread_result in results {
        for (station_name, stats) in thread_result {
            merged_stations
                .entry(station_name)
                .and_modify(|existing| existing.merge(&stats))
                .or_insert(stats);
        }
    }

    merged_stations
}

fn print_results(stations: &HashMap<String, StationStats>) {
    println!("Temperature Statistics by Weather Station:");
    println!("{{");

    let mut station_names: Vec<_> = stations.keys().collect();
    station_names.sort();

    for (i, name) in station_names.iter().enumerate() {
        let stats = &stations[*name];
        print!(
            "{}={:.1}/{:.1}/{:.1}",
            name,
            stats.min_temp,
            stats.mean(),
            stats.max_temp
        );

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
        println!("Usage: {} <path_to_weather_stations.txt>", args[0]);
        println!("Reading from default path: assets/weather_stations.csv");
        PathBuf::from("assets/weather_stations.csv")
    };

    println!("Processing file: {}", path.display());

    // Use parallel processing for better performance
    let stations = process_file_parallel(&path);

    println!("Processed {} unique weather stations", stations.len());

    print_results(&stations);
}
