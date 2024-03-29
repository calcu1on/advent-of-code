use crate::utils::read_lines;

const RED_MAX: u32 = 12;
const GREEN_MAX: u32 = 13;
const BLUE_MAX: u32 = 14;

static mut RED_MIN: u32 = 0;
static mut GREEN_MIN: u32 = 0;
static mut BLUE_MIN: u32 = 0;

pub fn process() -> u32 {
    let file_path = "./input_sources/day2short.txt";
    let mut total: u32 = 0;
    if let Ok(lines) = read_lines(file_path) {
        for game in lines.flatten() {
            check_if_game_is_valid(game.clone());
            // println!("{:?}", is_game_valid);
            if is_game_valid {
                let game_id: u32 = get_game_id(game.clone());
                total = total + game_id;
            }
        }
    }
    total
}


fn get_game_id(input_line: String) -> u32 {
    let game_section: Vec<&str> = input_line.split(":").collect();
    let game_section_split: Vec<&str> = game_section[0].split(" ").collect();
    game_section_split[1].trim().parse().unwrap()
}

fn check_if_game_is_valid(game: String) -> bool {
    let split_on_colon: Vec<&str> = game.split(":").collect();
    let game_results: Vec<&str> = split_on_colon[1].split(";").collect();
    let mut valid_game: bool = true;

    for game_result in game_results {
        let selections: Vec<&str> = game_result.split(",").collect();
        for selection in selections {
            let split_selection: Vec<&str> = selection.trim().split(" ").collect();
            let num_selected: u32 = split_selection[0].trim().parse().unwrap();
            let color_selected: &str = split_selection[1].trim();

            match color_selected {
                "green" => {
                    if num_selected > GREEN_MIN {
                        GREEN_MIN = num_selected;
                    }
                },
                "blue" => {
                    if num_selected > BLUE_MIN {
                        BLUE_MIN = num_selected;
                    }
                },
                "red" => {
                    if num_selected > RED_MIN {
                        RED_MIN = num_selected;
                    }
                },
                &_ => todo!()
            }
        }
    }
    valid_game
}

