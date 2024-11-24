use crate::utils::read_lines;

const RED_MAX: u32 = 12;
const GREEN_MAX: u32 = 13;
const BLUE_MAX: u32 = 14;

pub fn process() -> u32 {
    let file_path = "./input_sources/day2.txt";
    let mut total: u32 = 0;
    if let Ok(lines) = read_lines(file_path) {
        for game in lines.flatten() {
            total = total + get_color_minimums(game.clone());
            //println!("{:?}", total);
        }
    }
    total
}

fn get_game_id(input_line: String) -> u32 {
    let game_section: Vec<&str> = input_line.split(":").collect();
    let game_section_split: Vec<&str> = game_section[0].split(" ").collect();
    game_section_split[1].trim().parse().unwrap()
}

fn get_color_minimums(game: String) -> u32 {
    let split_on_colon: Vec<&str> = game.split(":").collect();
    let game_results: Vec<&str> = split_on_colon[1].split(";").collect();
    let mut valid_game: bool = true;

    let mut total: u32 = 1;
    let mut green_min: u32 = 0;
    let mut red_min: u32 = 0;
    let mut blue_min: u32 = 0;

    for game_result in game_results {
        let selections: Vec<&str> = game_result.split(",").collect();
        for selection in selections {
            let split_selection: Vec<&str> = selection.trim().split(" ").collect();
            let num_selected: u32 = split_selection[0].trim().parse().unwrap();
            let color_selected: &str = split_selection[1].trim();

            match color_selected {
                "green" => {
                    if num_selected > green_min {
                        green_min = num_selected;
                    }
                },
                "blue" => {
                    if num_selected > blue_min {
                        blue_min = num_selected;
                    }
                },
                "red" => {
                    if num_selected > red_min {
                        red_min = num_selected;
                    }
                },
                &_ => todo!()
            }
        }

    }
    if green_min > 0 {
        total = green_min * total;
    }

    if red_min > 0 {
        total = red_min * total;
    }

    if blue_min > 0 {
        total = blue_min * total;
    }
    total
}

