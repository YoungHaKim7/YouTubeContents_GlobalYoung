use rayon::prelude::*;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, Clone)]
struct CountryStats {
    min: u64,
    max: u64,
    sum: u64,
    count: u64,
}

impl CountryStats {
    fn new(population: u64) -> Self {
        CountryStats {
            min: population,
            max: population,
            sum: population,
            count: 1,
        }
    }

    fn update(&mut self, population: u64) {
        self.min = self.min.min(population);
        self.max = self.max.max(population);
        self.sum += population;
        self.count += 1;
    }

    fn mean(&self) -> f64 {
        self.sum as f64 / self.count as f64
    }

    // Merge two CountryStats (for combining results from different threads)
    fn merge(&mut self, other: &CountryStats) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
        self.sum += other.sum;
        self.count += other.count;
    }
}

fn parse_csv_line(line: &str) -> Option<(&str, u64)> {
    // Skip header line
    if line.starts_with("\"city\"") {
        return None;
    }

    // Parse CSV line: "city","city_ascii","lat","lng","country","iso2","iso3","admin_name","capital","population","id"
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() >= 11 {
        let country = parts[4].trim_matches('"');
        let population_str = parts[9].trim_matches('"');

        // Parse population, skip if empty or invalid
        if let Ok(population) = population_str.parse::<u64>() {
            if population > 0 {
                return Some((country, population));
            }
        }
    }
    None
}

fn process_chunk(lines: Vec<String>) -> HashMap<String, CountryStats> {
    let mut countries: HashMap<String, CountryStats> = HashMap::new();

    for line in lines {
        if let Some((country_name, population)) = parse_csv_line(&line) {
            countries
                .entry(country_name.to_string())
                .and_modify(|stats| stats.update(population))
                .or_insert_with(|| CountryStats::new(population));
        }
    }

    countries
}

fn process_file_parallel(path: &PathBuf) -> HashMap<String, CountryStats> {
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
    let results: Vec<HashMap<String, CountryStats>> =
        chunks.into_par_iter().map(process_chunk).collect();

    // Merge results from all threads
    let mut merged_countries: HashMap<String, CountryStats> = HashMap::new();

    for thread_result in results {
        for (country_name, stats) in thread_result {
            merged_countries
                .entry(country_name)
                .and_modify(|existing| existing.merge(&stats))
                .or_insert(stats);
        }
    }

    merged_countries
}

fn print_results(countries: &HashMap<String, CountryStats>) {
    println!("Population Statistics by Country:");
    println!("{{");

    let mut country_names: Vec<_> = countries.keys().collect();
    country_names.sort();

    for (i, name) in country_names.iter().enumerate() {
        let stats = &countries[*name];
        print!(
            "{}={:.0}/{:.0}/{:.0}",
            name,
            stats.min,
            stats.mean(),
            stats.max
        );

        if i < country_names.len() - 1 {
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
        println!("Usage: {} <path_to_worldcities.csv>", args[0]);
        println!("Reading from default path: assets/worldcities.csv");
        PathBuf::from("assets/worldcities.csv")
    };

    println!("Processing file: {}", path.display());

    // Use parallel processing for better performance
    let countries = process_file_parallel(&path);

    println!("Processed {} unique countries", countries.len());

    print_results(&countries);
}
