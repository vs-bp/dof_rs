use std::{collections::HashMap, fs::File, hash::Hash, io::Read};
use byteable::Byteable;

#[derive(Byteable, Debug, Clone, PartialEq)]
#[byteable(io_only)]
pub struct Motor {
    pub name: String,
    pub manufacturer: String,
    pub diameter_mm: f32,
    pub length_mm: f32,
    pub thrust_curve: Vec<(f32, f32)>
}
impl Motor {
    pub fn default() -> Motor { Motor {
        name: "None".to_string(),
        manufacturer: "None".to_string(),
        diameter_mm: 0.0,
        length_mm: 0.0,
        thrust_curve: vec![]
    }}
    pub fn from_file(path: String, manufacturer: String) -> Vec<Motor> {
        // Turn input file into lines.
        let mut file: File = File::open(path).unwrap();
        let mut file_string: String = "".to_owned();
        file.read_to_string(&mut file_string).unwrap();
        let lines: Vec<&str> = file_string.lines().collect();
        
        // Read in motor specs line by line.
        let mut specs: HashMap<String, Motor> = HashMap::new();
        let mut reading_new: bool = true;
        let mut reading_name: String = "".to_string();
        for line in lines {
            // Split into words and skip empty and comment lines.
            let words: Vec<&str> = line.split_whitespace().collect();
            if words.len() == 0 { continue; }
            if words[0].contains(";") { continue; }

            if reading_new { 
                // If reading_new, then we push new motor with first word as motor name.
                specs.insert(words[0].to_string(), Motor { 
                    name: words[0].to_string(), 
                    manufacturer: manufacturer.clone(), 
                    diameter_mm: words[1].parse().unwrap(),
                    length_mm: words[2].parse().unwrap(),
                    thrust_curve: vec![] 
                });
                reading_new = false;
                reading_name = words[0].to_string();
            } else {
                // If not then we're already reading and motor and a point on its thrust curve.
                // Next reading must be a new motor if the current point has thrust 0.0 and time above 0.0.
                let time: f32 = words[0].parse().unwrap();
                let thrust: f32 = words[1].parse().unwrap();
                specs.get_mut(&reading_name).unwrap().thrust_curve.push((time, thrust));
                if time > 0.0 && thrust == 0.0 { reading_new = true; }
            }
        }

        // Filter alphabetically.
        let mut specs2: Vec<Motor> = specs.iter().map(|(f, v)| { v.clone() }).collect();
        specs2.sort_by_key(|f| f.name.clone());
        return specs2;
    }
}