use crate::cli_parser::Config;
use std::fs;

pub fn run(config: Option<Config>) -> i64 {
    let file_path = config.unwrap().file_path;
    let input_data = fs::read_to_string(file_path).expect("Error when reading file");

    let parsed_input = MapSystem::from(&input_data).unwrap();
    let result = parsed_input.find_lowest_location();
    println!("{}", &result);
    result
}

#[derive(Debug)]
struct MapSystem<'a> {
    pub seeds: Vec<i64>,
    pub maps: Vec<Map<'a>>,
}

impl<'a> MapSystem<'a> {
    pub fn from(contents: &str) -> Result<MapSystem, &'static str> {
        let mut seeds: Vec<i64> = Vec::new();
        let mut maps: Vec<Map> = Vec::new();
        let mut cur_map: Option<Map> = None;
        let mut prev_map_dest_id: Option<&str> = None;

        for line in contents.lines().filter(|l| !l.is_empty()) {
            let parsed_line: Vec<&str> = line.split_whitespace().collect();

            if let Some(map_id_line) = parsed_line.get(0) {
                if map_id_line == &"seeds:" {
                    seeds = parsed_line[1..]
                        .iter()
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect();
                } else if map_id_line.contains("-to-") {
                    if let Some(m) = cur_map {
                        prev_map_dest_id = Some(m.dest_id);
                        maps.push(m.clone());
                    }
                    let parsed_map_ids: Vec<&str> = map_id_line
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .get(0)
                        .unwrap()
                        .split("-to-")
                        .collect();
                    cur_map = Some(Map::from(parsed_map_ids[0], parsed_map_ids[1], vec![]));
                    if prev_map_dest_id.is_some()
                        && prev_map_dest_id != Some(cur_map.clone().unwrap().src_id)
                    {
                        return Err("The maps order is incorrect");
                    }
                } else {
                    match cur_map {
                        Some(ref mut m) => {
                            let parsed_element: Vec<i64> = parsed_line
                                .iter()
                                .map(|s| s.parse::<i64>().unwrap())
                                .collect();
                            let map_element: MapElement = MapElement {
                                dest: *parsed_element.get(0).unwrap(),
                                src: *parsed_element.get(1).unwrap(),
                                range: *parsed_element.get(2).unwrap(),
                            };
                            m.elements.push(map_element);
                        }
                        None => {
                            return Err("Line does not belong to any map");
                        }
                    }
                }
            }
        }
        if let Some(m) = cur_map {
            maps.push(m);
        }

        Ok(MapSystem { maps, seeds })
    }

    pub fn find_lowest_location(&self) -> i64 {
        let mut lowest_loc: i64 = -1;

        for seed in &self.seeds {
            let loc = self.find_location(*seed);
            if loc < lowest_loc || lowest_loc == -1 {
                lowest_loc = loc;
            }
        }
        lowest_loc
    }

    fn find_location(&self, seed: i64) -> i64 {
        let mut cur_dest: i64 = seed;

        for map in &self.maps {
            for el in &map.elements {
                if let Some(d) = el.get_next_destination(cur_dest) {
                    cur_dest = d;
                    break;
                }
            }
        }
        cur_dest
    }
}

#[derive(Debug, Clone)]
struct MapElement {
    pub dest: i64,
    pub src: i64,
    pub range: i64,
}

impl MapElement {
    pub fn get_next_destination(&self, src: i64) -> Option<i64> {
        if self.includes(src) {
            return Some(src + (self.dest - self.src));
        }
        None
    }

    fn includes(&self, x: i64) -> bool {
        x >= self.src && x <= self.src + self.range - 1
    }
}

#[derive(Debug, Clone)]
struct Map<'a> {
    src_id: &'a str,
    dest_id: &'a str,
    elements: Vec<MapElement>,
}

impl<'a> Map<'a> {
    pub fn from(src_id: &'a str, dest_id: &'a str, elements: Vec<MapElement>) -> Map<'a> {
        Map {
            src_id,
            dest_id,
            elements,
        }
    }
}
