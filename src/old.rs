use std::fs;
use std::path::Path;
use pbr::ProgressBar;

fn main() {
    
    let path = Path::new("./stats/");

    if !path.is_dir() {
        println!("unable to read states directory");
    }

    let dir = path.read_dir().unwrap();
    let dir_len = path.read_dir().unwrap().count();
    let mut pb = ProgressBar::new(dir_len as u64);

    // let mut heighest_jumps = 0;
    // let mut heighest_jumps_user = String::new();
    let mut heighest_jumps: Vec<(String, u64)> = Vec::new();
    for entry in dir {
        pb.inc();
        let entry = entry.unwrap();
        let json: serde_json::Value = if let Ok(json) = serde_json::from_str(&fs::read_to_string(entry.path()).unwrap()) {
            json
        } else {
            println!("Error reading json at path {}", entry.path().to_str().unwrap());
            continue;
        };
        if let Some(jumps) = json["stats"]["minecraft:custom"].get("minecraft:jump") {
            let jumps = jumps.as_u64().unwrap();
            // if heighest_jumps.len() < 1 || jumps > heighest_jumps[0].1 {
            heighest_jumps.push((entry.file_name().into_string().unwrap(), jumps));
            // }

            // if heighest_jumps.len() > 10 {
            //     heighest_jumps.pop();
            // }
        }
    }
    pb.finish();

    heighest_jumps.sort_by_key(|(_, jumps)| *jumps);
    heighest_jumps.reverse();

    for (i, (mut name, jumps)) in heighest_jumps.into_iter().enumerate() {
        if i > 10 {
            break;
        }
        name.replace_range(name.len()-5.., "");
        println!("{}. {} with {} jumps", i + 1, name, jumps);
    }
    

    // pb.finish_println(&format!("Finished reader player statistics, the uuid of the player with most jumps is {} with {} jumps.", heighest_jumps_user, heighest_jumps));

}