use bevy::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use crate::game::player::events::OnPlayerDeath;

use super::*;
use crate::ui::events::OnPlayButtonClicked;

const PATH: &str = "./player_data.txt" ;

pub fn update_score_increment_timer(
    mut score_increment_timer: ResMut<ScoreIncrementTimer>,
    time: Res<Time>
) {
    score_increment_timer.timer.tick(time.delta());
}

pub fn increase_score(
    score_increment_timer: Res<ScoreIncrementTimer>,
    mut score: ResMut<Score>
) {
    if score_increment_timer.timer.finished() {
        score.value += score.score_delta;
    }
}

pub fn refresh_score_on_player_death(
    mut score: ResMut<Score>,
    mut event: EventReader<OnPlayButtonClicked>
) {
    for _ in event.read() {
        if score.value != 0 {
            score.value = 0;
        }
    }
}

pub fn create_player_log_file(mut score: ResMut<Score>) {

    match File::open(PATH) {
        Ok(_) => {
            let previous_score = read_info();
            println!("the file was found, score: {previous_score}");
            score.highest_score = previous_score;
        },
        Err(_) => {
            let mut f = File::create(PATH).expect("Could not create a file");
            f.write_all(b"0");
        }
    }

}

pub fn save_score_in_file(mut score: ResMut<Score>) {
    let last_score: u32 = read_info();

    if (last_score < score.value) {
        save_info(score.value.to_string());
        score.highest_score = score.value;
    }

    println!("HIGH SCORE: {}", score.highest_score);
}

fn save_info(text: String){

    let mut file: File = File::create(PATH)
        .expect("Could not create the log file!");

    file.write_all(text.as_bytes())
        .expect("Could not write to the log file!");
}

fn read_info() -> u32 {
    let mut file = File::open(PATH).expect("Could not find the log file!");

    let mut last_score = String::new();

    file.read_to_string(&mut last_score)
        .expect("Cannot read the file!");
    println!("{}", last_score);
    last_score.parse().expect("Segmentaion fault")
}
