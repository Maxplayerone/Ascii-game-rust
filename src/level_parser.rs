use std::fs::File;
use std::io::{self, BufRead};

use crate::{chest, math, weapons};

pub struct ParserInfo {
    pub player: math::Pos2,
    pub map_dimensions: math::Pos2,
    pub enemies: Vec<math::Pos2>,
    pub chests: Vec<chest::Chest>,
    pub unbreakable: Vec<math::Pos2>,
}

impl ParserInfo {
    fn new() -> Self {
        Self {
            player: math::Pos2::new(0, 0),
            map_dimensions: math::Pos2::new(0, 0),
            enemies: Vec::new(),
            chests: Vec::new(),
            unbreakable: Vec::new(),
        }
    }
}

pub fn parse_level(
    player_symbol: char,
    enemy_symbol: char,
    chest_symbol: char,
    unbreakable_symbol: char,
) -> (Vec<char>, ParserInfo) {
    // Open the file in read-only mode
    let file = File::open("src/level_one.sag").unwrap();

    // Create a buffer reader to read the file line by line
    let reader = io::BufReader::new(file);

    // Iterate over each line in the file
    let mut map: Vec<char> = Vec::new();
    let mut info: ParserInfo = ParserInfo::new();

    let mut i: i32 = 0;
    let mut checked_line_size_flag = false;
    let mut map_width: i32 = 0;
    let mut height_count: i32 = 0;

    for line in reader.lines() {
        height_count += 1;

        let line: String = line.unwrap();

        if !checked_line_size_flag {
            map_width = line.chars().count() as i32;
            checked_line_size_flag = true;
        }

        for c in line.chars() {
            if c == player_symbol {
                let x: i32 = i % map_width;
                let y: i32 = i / map_width;
                info.player = math::Pos2::new(x, y);
            } else if c == enemy_symbol {
                let x: i32 = i % map_width;
                let y: i32 = i / map_width;
                info.enemies.push(math::Pos2::new(x, y));
            } else if c == chest_symbol {
                let x: i32 = i % map_width;
                let y: i32 = i / map_width;
                info.chests.push(chest::Chest::new(
                    math::Pos2::new(x, y),
                    weapons::ItemType::Rifle,
                ));
            } else if c == unbreakable_symbol {
                let x: i32 = i % map_width;
                let y: i32 = i / map_width;
                info.unbreakable.push(math::Pos2::new(x, y));
            }
            map.push(c);
            i += 1;
        }
        map.push('\n');
    }
    info.map_dimensions = math::Pos2::new(map_width, height_count);
    (map, info)
}
