#include <algorithm>
#include <filesystem>
#include <format>
#include <fstream>
#include <iostream>
#include <mutex>
#include <ranges>
#include <sstream>
#include <string>
#include <thread>
#include <unordered_map>
#include <vector>

struct StationStats {
    double min_temp;
    double max_temp;
    double sum_temp;
    uint64_t count;

    StationStats(double temperature)
        : min_temp(temperature), max_temp(temperature), sum_temp(temperature),
          count(1) {}

    void update(double temperature) {
        min_temp = std::min(min_temp, temperature);
        max_temp = std::max(max_temp, temperature);
        sum_temp += temperature;
        ++count;
    }

    double mean() const { return sum_temp / static_cast<double>(count); }

    void merge(const StationStats &other) {
        min_temp = std::min(min_temp, other.min_temp);
        max_temp = std::max(max_temp, other.max_temp);
        sum_temp += other.sum_temp;
        count += other.count;
    }
};

std::pair<std::string, double> parseLine(const std::string &line) {
    if (line.empty() || line[0] == '#') {
        return {"", 0.0};
    }

    size_t semicolon = line.find(';');
    if (semicolon != std::string::npos) {
        std::string station_name = line.substr(0, semicolon);
        std::string temp_str = line.substr(semicolon + 1);

        // Trim whitespace
        auto not_space = [](char c) {
            return !std::isspace(static_cast<unsigned char>(c));
        };
        station_name.erase(
            station_name.begin(),
            std::find_if(station_name.begin(), station_name.end(), not_space));
        station_name.erase(
            std::find_if(station_name.rbegin(), station_name.rend(), not_space)
                .base(),
            station_name.end());

        temp_str.erase(
            temp_str.begin(),
            std::find_if(temp_str.begin(), temp_str.end(), not_space));
        temp_str.erase(
            std::find_if(temp_str.rbegin(), temp_str.rend(), not_space).base(),
            temp_str.end());

        try {
            double temperature = std::stod(temp_str);
            return {station_name, temperature};
        } catch (...) {
            return {"", 0.0};
        }
    }

    return {"", 0.0};
}

using StationMap = std::unordered_map<std::string, StationStats>;

StationMap processChunk(const std::vector<std::string> &lines) {
    StationMap stations;

    for (const auto &line : lines) {
        auto [station_name, temperature] = parseLine(line);
        if (!station_name.empty()) {
            auto it = stations.find(station_name);
            if (it != stations.end()) {
                it->second.update(temperature);
            } else {
                stations.emplace(station_name, StationStats(temperature));
            }
        }
    }

    return stations;
}

StationMap processFileParallel(const std::filesystem::path &path) {
    std::ifstream file(path);
    if (!file) {
        throw std::runtime_error("Failed to open file");
    }

    std::vector<std::string> lines;
    std::string line;
    while (std::getline(file, line)) {
        lines.push_back(line);
    }

    std::cout << std::format("Read {} lines from file\n", lines.size());

    // Split into chunks for parallel processing
    const auto num_threads = std::thread::hardware_concurrency();
    size_t chunk_size = (lines.size() + num_threads - 1) / num_threads;

    std::vector<std::vector<std::string>> chunks;
    for (size_t i = 0; i < lines.size(); i += chunk_size) {
        auto end = std::min(i + chunk_size, lines.size());
        chunks.emplace_back(lines.begin() + i, lines.begin() + end);
    }

    std::cout << std::format("Processing {} chunks using {} threads\n",
                             chunks.size(), num_threads);

    // Process chunks in parallel
    std::vector<StationMap> results(chunks.size());
    std::vector<std::thread> threads;

    for (size_t i = 0; i < chunks.size(); ++i) {
        threads.emplace_back(
            [i, &chunks, &results]() { results[i] = processChunk(chunks[i]); });
    }

    for (auto &thread : threads) {
        thread.join();
    }

    // Merge results from all threads
    StationMap merged_stations;

    for (const auto &thread_result : results) {
        for (const auto &[station_name, stats] : thread_result) {
            auto it = merged_stations.find(station_name);
            if (it != merged_stations.end()) {
                it->second.merge(stats);
            } else {
                merged_stations.emplace(station_name, stats);
            }
        }
    }

    return merged_stations;
}

void printResults(const StationMap &stations) {
    std::cout << "Temperature Statistics by Weather Station:\n";
    std::cout << "{\n";

    std::vector<std::string> station_names;
    station_names.reserve(stations.size());
    for (const auto &[name, _] : stations) {
        station_names.push_back(name);
    }
    std::sort(station_names.begin(), station_names.end());

    for (size_t i = 0; i < station_names.size(); ++i) {
        const auto &name = station_names[i];
        const auto &stats = stations.at(name);
        std::cout << std::format("{}={:.1}/{:.1}/{:.1}", name, stats.min_temp,
                                 stats.mean(), stats.max_temp);

        if (i < station_names.size() - 1) {
            std::cout << ", ";
        }
    }

    std::cout << "\n}\n";
}

int main(int argc, char *argv[]) {
    std::filesystem::path path;

    if (argc > 1) {
        path = argv[1];
    } else {
        std::cout << std::format("Usage: {} <path_to_weather_stations.txt>\n",
                                 argv[0]);
        std::cout << "Reading from default path: assets/weather_stations.csv\n";
        path = "assets/weather_stations.csv";
    }

    std::cout << std::format("Processing file: {}\n", path.string());

    try {
        auto stations = processFileParallel(path);

        std::cout << std::format("Processed {} unique weather stations\n",
                                 stations.size());

        printResults(stations);
    } catch (const std::exception &e) {
        std::cerr << std::format("Error: {}\n", e.what());
        return 1;
    }

    return 0;
}
