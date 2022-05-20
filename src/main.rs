use serde_json::{from_str, Value};
use std::{collections::HashMap, fs};

fn main() {
    let data = fs::read_to_string("./src/data.json").expect("Error reading data.json");
    let data: Value = from_str(&data).expect("Error parsing data.json");

    //convert data as Hashmap<String,Hashmap<String,f64>>
    let mut data_map: HashMap<String, HashMap<String, f64>> = HashMap::new();
    for (key, value) in data.as_object().unwrap().iter() {
        let mut inner_map: HashMap<String, f64> = HashMap::new();
        for (key2, value2) in value.as_object().unwrap().iter() {
            inner_map.insert(key2.to_string(), value2.as_f64().unwrap());
        }
        data_map.insert(key.to_string(), inner_map);
    }

    println!("{:?}", top_matches(&data_map, "Toby", 6));
}

fn pearson(prefs: &HashMap<String, HashMap<String, f64>>, person1: &str, person2: &str) -> f64 {
    //Get the list of mutually rated items
    let mut si: HashMap<String, u16> = HashMap::new();
    for (key, _value) in prefs.get(person1).unwrap().iter() {
        if prefs.get(person2).unwrap().contains_key(key) {
            si.insert(key.to_string(), 1);
        }
    }

    //if they have no ratings in common, return 0
    if si.len() == 0 {
        return 0.0;
    }

    // Sum calculations
    let n: f64 = si.len() as f64;

    // Sums of all the preferences intersecting si
    let sum1: f64 = si.iter().fold(0.0, |sum, (key, _value)| {
        sum + prefs.get(person1).unwrap().get(key).unwrap()
    });

    let sum2: f64 = si.iter().fold(0.0, |sum, (key, _value)| {
        sum + prefs.get(person2).unwrap().get(key).unwrap()
    });

    // Sum up the squares of all the preferences intersecting si
    let sum1sq: f64 = si.iter().fold(0.0, |sum, (key, _value)| {
        sum + prefs.get(person1).unwrap().get(key).unwrap().powf(2.0)
    });

    let sum2sq: f64 = si.iter().fold(0.0, |sum, (key, _value)| {
        sum + prefs.get(person2).unwrap().get(key).unwrap().powf(2.0)
    });

    // Sum up the products
    let mut p_sum: f64 = 0.0;
    for (key, _value) in si.iter() {
        p_sum += prefs.get(person1).unwrap().get(key).unwrap()
            * prefs.get(person2).unwrap().get(key).unwrap();
    }

    // Calculate Pearson score
    let num: f64 = p_sum - (sum1 * sum2 / n as f64);
    let den: f64 =
        ((sum1sq - (sum1 * sum1 / n as f64)) * (sum2sq - (sum2 * sum2 / n as f64))).sqrt();

    if den == 0.0 {
        return 0.0;
    }

    num / den
}

fn top_matches(
    prefs: &HashMap<String, HashMap<String, f64>>,
    person: &str,
    n: u16,
) -> Vec<(String, f64)> {
    let mut scores: Vec<(String, f64)> = Vec::new();
    for (key, _value) in prefs.iter() {
        if key != person {
            scores.push((key.to_string(), pearson(prefs, person, key)));
        }
    }

    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    scores.truncate(n as usize);
    scores
}
