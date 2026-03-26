use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
struct ManeuverJson {
    #[serde(rename = "vehicleType")]
    vehicle_type: String,
    name: String,
    description: String,
    difficulty: i32,
    tags: Vec<String>,
}

fn main() {
    let text_dir = Path::new("../text");
    let video_dir = Path::new("../videos");

    if !text_dir.exists() {
        eprintln!("Text directory not found at {:?}", text_dir);
        return;
    }

    let mut maneuvers = Vec::new();
    let mut unique_tags = HashSet::new();

    // Read all json files
    for entry in fs::read_dir(text_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path).unwrap();
            let maneuver: ManeuverJson = serde_json::from_str(&content).expect(&format!("Failed to parse {:?}", path));
            
            for tag in &maneuver.tags {
                unique_tags.insert(tag.clone());
            }

            let file_stem = path.file_stem().unwrap().to_str().unwrap().to_string();
            maneuvers.push((file_stem, maneuver));
        }
    }

    // Map tags to UUIDs
    let mut tag_uuids = HashMap::new();
    for tag in unique_tags {
        tag_uuids.insert(tag, Uuid::new_v4());
    }

    println!("BEGIN;\n");
    println!("TRUNCATE TABLE maneuver.maneuver_tag CASCADE;");
    println!("TRUNCATE TABLE maneuver.tag CASCADE;");
    println!("TRUNCATE TABLE maneuver.maneuver CASCADE;\n");

    // Insert tags
    for (tag, id) in &tag_uuids {
        let safe_tag = tag.replace("'", "''");
        println!("INSERT INTO maneuver.tag (id, name) VALUES ('{}', '{}');", id, safe_tag);
    }
    println!();

    // Discover videos
    let mut videos = HashMap::new();
    if video_dir.exists() {
        for entry in fs::read_dir(video_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                    videos.insert(file_stem.to_string(), file_name.to_string());
                }
            }
        }
    }

    // Insert maneuvers and maneuver_tags
    for (file_stem, maneuver) in maneuvers {
        let maneuver_id = Uuid::new_v4();
        
        let video_path = if let Some(video_filename) = videos.get(&file_stem) {
            format!("'{}'", video_filename)
        } else {
            "NULL".to_string()
        };

        let safe_name = maneuver.name.replace("'", "''");
        let safe_desc = maneuver.description.replace("'", "''");

        println!(
            "INSERT INTO maneuver.maneuver (id, vehicle_type, name, description, difficulty, video_path) VALUES ('{}', '{}', '{}', '{}', {}, {});",
            maneuver_id, maneuver.vehicle_type, safe_name, safe_desc, maneuver.difficulty, video_path
        );

        for tag in maneuver.tags {
            let tag_id = tag_uuids.get(&tag).unwrap();
            println!("INSERT INTO maneuver.maneuver_tag (maneuver_id, tag_id) VALUES ('{}', '{}');", maneuver_id, tag_id);
        }
        println!();
    }

    println!("COMMIT;");
}
