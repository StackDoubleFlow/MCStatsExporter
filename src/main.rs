use std::fs;
use std::path::Path;
use pbr::ProgressBar;

const STATISTICS: &'static [&str] = &[
    "minecraft:animals_bred",
    "minecraft:clean_armor",
    "minecraft:clean_banner",
    "minecraft:open_barrel",
    "minecraft:bell_ring",
    "minecraft:eat_cake_slice",
    "minecraft:fill_cauldron",
    "minecraft:open_chest",
    "minecraft:damage_absorbed",
    "minecraft:damage_blocked_by_shield",
    "minecraft:damage_dealt",
    "minecraft:damage_dealt_absorbed",
    "minecraft:damage_dealt_resisted",
    "minecraft:damage_resisted",
    "minecraft:damage_taken",
    "minecraft:inspect_dispenser",
    "minecraft:climb_one_cm",
    "minecraft:crouch_one_cm",
    "minecraft:fall_one_cm",
    "minecraft:fly_one_cm",
    "minecraft:sprint_one_cm",
    "minecraft:swim_one_cm",
    "minecraft:walk_one_cm",
    "minecraft:walk_on_water_one_cm",
    "minecraft:walk_under_water_one_cm",
    "minecraft:boat_one_cm",
    "minecraft:aviate_one_cm",
    "minecraft:horse_one_cm",
    "minecraft:minecart_one_cm",
    "minecraft:pig_one_cm",
    "minecraft:strider_one_cm",
    "minecraft:inspect_dropper",
    "minecraft:open_enderchest",
    "minecraft:fish_caught",
    "minecraft:leave_game",
    "minecraft:inspect_hopper",
    "minecraft:interact_with_anvil",
    "minecraft:interact_with_beacon",
    "minecraft:interact_with_blast_furnace",
    "minecraft:interact_with_brewingstand",
    "minecraft:interact_with_campfire",
    "minecraft:interact_with_cartography_table",
    "minecraft:interact_with_crafting_table",
    "minecraft:interact_with_furnace",
    "minecraft:interact_with_gridstone",
    "minecraft:interact_with_lectern",
    "minecraft:interact_with_loom",
    "minecraft:interact_with_smithing_table",
    "minecraft:interact_with_smoker",
    "minecraft:interact_with_stonecutter",
    "minecraft:drop",
    "minecraft:enchant_item",
    "minecraft:jump",
    "minecraft:mob_kills",
    "minecraft:play_record",
    "minecraft:play_noteblock",
    "minecraft:tune_noteblock",
    "minecraft:deaths",
    "minecraft:pot_flower",
    "minecraft:player_kills",
    "minecraft:raid_trigger",
    "minecraft:raid_win",
    "minecraft:clean_shulker_box",
    "minecraft:open_shulker_box",
    "minecraft:sneak_time",
    "minecraft:talked_to_villager",
    "minecraft:target_hit",
    "minecraft:play_one_minute",
    "minecraft:time_since_death",
    "minecraft:time_since_rest",
    "minecraft:sleep_in_bed",
    "minecraft:traded_with_villager",
    "minecraft:trigger_trapped_chest",
    "minecraft:use_cauldron"
];

fn main() {
    let path = Path::new("./stats/");

    if !path.is_dir() {
        println!("unable to read states directory");
    }

    let dir = path.read_dir().unwrap();
    let dir_len = path.read_dir().unwrap().count();
    let mut pb = ProgressBar::new(dir_len as u64);

    let mut wtr = csv::WriterBuilder::new().from_path("stats.csv").unwrap();
    let mut title = Vec::from(STATISTICS);
    title.insert(0, "uuid");
    wtr.write_record(&title).unwrap();

    for entry in dir {
        pb.inc();
        let entry = entry.unwrap();
        let json: serde_json::Value = if let Ok(json) = serde_json::from_str(&fs::read_to_string(entry.path()).unwrap()) {
            json
        } else {
            println!("Error reading json at path {}", entry.path().to_str().unwrap());
            continue;
        };

        let mut uuid = entry.file_name().into_string().unwrap();
        uuid.replace_range(uuid.len()-5.., "");
        let mut stats: Vec<String> = Vec::new();
        stats.push(uuid);
        for stat_name in STATISTICS {
            if let Some(stat) = json["stats"]["minecraft:custom"].get(*stat_name) {
                stats.push(stat.to_string())
            } else {
                stats.push(String::from("0"));
            }
        }
        wtr.write_record(&stats).unwrap();
    }
    wtr.flush().unwrap();
    pb.finish();
}