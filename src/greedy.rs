// This is a Greedy algorithm for the Travelling Salesman Problem.
// It is a simple algorithm that is not guaranteed to find the optimal solution.
// It is a good algorithm to use as a baseline for comparison.
// It is also a good algorithm to use when the number of cities is small.

use std::cmp::Ordering;
use std::collections::HashSet;
use std::f64::INFINITY;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::city::City;
use crate::tour::Tour;

pub fn greedy(cities: &Vec<City>) -> Tour {
    let mut tour = Tour::new();
    let mut visited = HashSet::new();
    let mut current_city = &cities[0];
    visited.insert(current_city);
    tour.add_city(current_city);
    while visited.len() < cities.len() {
        let mut closest_city = &cities[0];
        let mut closest_distance = INFINITY;
        for city in cities {
            if visited.contains(city) {
                continue;
            }
            let distance = current_city.distance_to(city);
            match distance.partial_cmp(&closest_distance) {
                Some(Ordering::Less) => {
                    closest_city = city;
                    closest_distance = distance;
                }
                _ => (),
            }
        }
        visited.insert(closest_city);
        tour.add_city(closest_city);
        current_city = closest_city;
    }
    tour
}

pub fn greedy_from_file(path: &Path) -> Tour {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut cities = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(',').collect();
        let city = City::new(parts[0].parse().unwrap(), parts[1].parse().unwrap());
        cities.push(city);
    }
    greedy(&cities)
}

//