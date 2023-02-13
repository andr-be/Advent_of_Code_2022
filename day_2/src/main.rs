/// Advent of Code 2022: Day 2, Parts 1 & 2 Solution
/// andr-be 2023
///
/// There's a lot going on in this program, and I'm not currently in the state of mind to explain
/// it properly. I'll briefly try and outline the flow:
///
/// 1) read the input file, split it into lines, enumerate and begin to iterate over them 
///
/// 2) take the first character and add it to the 'opponent_move' column of a new Game
///
/// 3) take the 3rd character and translate it twice;
///     - once into the Part 1 code, rendering it into a move
///     - once into the Part 2 code, turning it into an outcome
///
/// 4) calculate the scores for both of these potential outcomes
///     - score_1 plays it straight, we just play out every games based on our move vs. theirs
///     - mod_score takes a different tack and works backwards from the outcome
///
/// 5) tot up all of the scores into a Totals struct, display scores in the terminal

use std::path::Path;
#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

struct Totals {
    matches: usize,
    wins: usize,
    losses: usize,
    score: usize,
    wlr: f64,
    mod_score: usize,
    mod_wins: usize,
    mod_losses: usize,
    mod_wlr: f64,
}

struct Game {
    game_id: usize,
    opponent_move: Move,
    your_move: Move,
    outcome: Outcome,
    mod_outcome: Outcome,
    score: usize,
    mod_move: Move,
    mod_score: usize,
}

impl Game {
    fn calculate_score(&mut self) {
        self.score = match &self.your_move {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };

        match (&self.opponent_move, &self.your_move) {
            (Move::Rock, Move::Rock) | 
            (Move::Paper, Move::Paper) | 
            (Move::Scissors, Move::Scissors) => {
                self.score += 3; 
                self.outcome = Outcome::Draw
            },

            (Move::Rock, Move::Paper) | 
            (Move::Paper, Move::Scissors) | 
            (Move::Scissors, Move::Rock) => {
                self.score += 6; 
                self.outcome = Outcome::Win
            },

            (Move::Rock, Move::Scissors) | 
            (Move::Paper, Move::Rock) | 
            (Move::Scissors, Move::Paper) => {
                self.score += 0; 
                self.outcome = Outcome::Lose
            },
        };
    }

    fn calculate_modified_score(&mut self) {
        self.mod_score = match &self.mod_outcome {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        };

        match (&self.mod_outcome, &self.opponent_move) {
                (Outcome::Win, Move::Rock) | 
                (Outcome::Draw, Move::Paper) | 
                (Outcome::Lose, Move::Scissors) => {
                    self.mod_move = Move::Paper;
                    self.mod_score += 2;
                },
                
                (Outcome::Win, Move::Paper) | 
                (Outcome::Draw, Move::Scissors) | 
                (Outcome::Lose, Move::Rock) => {
                    self.mod_move = Move::Scissors;
                    self.mod_score += 3;
                },

                (Outcome::Win, Move::Scissors) | 
                (Outcome::Draw, Move::Rock) | 
                (Outcome::Lose, Move::Paper) => {
                    self.mod_move = Move::Rock;
                    self.mod_score += 1;
                },
            }
    }
}

fn read_input(path: &Path) -> String {
    let ret = match std::fs::read_to_string(path) {
        Ok(input) => input,
        Err(_) => String::from(""),
    };
    ret
}

fn translate_code(code: char) -> Move {
    match code {
        'A' | 'X' => Move::Rock,
        'B' | 'Y' => Move::Paper,
        'C' | 'Z' => Move::Scissors,
        _ => panic!(),
    }
}

fn translate_code_v2(code: char) -> Outcome {
    match code {
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!(),
    }
}

fn main() {
    let path = Path::new("src/input.txt");

    //let input = String::from("A Y\nB X\nC Z");
    let input = read_input(path);

    let lines = input.lines();
    
    let mut games: Vec<Game> = Vec::new();

    for (game_id, line) in lines.enumerate() {
    
        let characters = line.chars().enumerate();
        
        let mut new_game = Game {
            game_id: game_id + 1,
            opponent_move: Move::Rock,
            your_move: Move::Rock,
            outcome: Outcome::Draw,
            mod_outcome: Outcome::Draw,
            score: 0,
            mod_move: Move::Rock,
            mod_score: 0,
        };
        
        for (position, letter) in characters {
            match position {
                0 => new_game.opponent_move = translate_code(letter),
                2 => {  
                        new_game.your_move = translate_code(letter); 
                        new_game.mod_outcome = translate_code_v2(letter)
                    },
                _ => {},
            }
        }
        
        new_game.calculate_score();
        new_game.calculate_modified_score();
        games.push(new_game);
    }

    let mut totals = Totals { 
        matches: 0,
        wins: 0, 
        losses: 0, 
        score: 0,
        wlr: 0.0, 
        mod_score: 0,
        mod_wins: 0,
        mod_losses: 0,
        mod_wlr: 0.0,
    };

    for game in games {
        totals.matches = game.game_id;
        totals.score += game.score;
        totals.mod_score += game.mod_score;
        match game.outcome {
            Outcome::Win => totals.wins += 1,
            Outcome::Lose => totals.losses += 1,
            Outcome::Draw => {},
        }
        match game.mod_outcome {
            Outcome::Win => totals.mod_wins += 1,
            Outcome::Lose => totals.mod_losses += 1,
            Outcome::Draw => {},
        }
        println!("Game: {}\tResult: {:?}\tScore: {:?}\tPart2-> R: {:?}, S:{:?}", 
            game.game_id, game.outcome, game.score,
            game.mod_outcome, game.mod_score);
    }

    totals.wlr = totals.wins as f64 / totals.losses as f64;
    totals.mod_wlr = totals.mod_wins as f64 / totals.mod_losses as f64;

    let win_pct: f64 = (totals.wins as f64 / totals.matches as f64) * 100.0;
    let mod_win_pct: f64 = (totals.wins as f64 / totals.matches as f64) * 100.0;

    println!("\nPART 1 SOLUTION:\nScore: {}\nWin/Loss: {:.2}\nWin %: {}\n", 
        totals.score, totals.wlr, win_pct);

    println!("PART 2 SOLUTION:\nModified Score: {:?}\nWin/Loss: {:.2}\nWin %: {}\n", 
        totals.mod_score, totals.mod_wlr, mod_win_pct);
}

