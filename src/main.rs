// Apologies to all who care -- I'm just used to camelCase
#![allow(non_snake_case)]

use std::io;

struct Player {
    name: String,
    scores: Vec<i32>,
    totalScore: i32,
}

// Assign a player an additional score of `score` for a hole
impl Player {
    fn justScored(&mut self, score: i32) {
        self.scores.push(score);
        self.totalScore += score;
    }
}

// Scan a string from `stdin`
fn scanString() -> String {
    loop {
        let mut scannedLine = String::new();

        io::stdin().read_line(&mut scannedLine).expect(
            "Failed to read line",
        );

        return scannedLine.trim().to_string();
    }
}

// Scan an `i32` from `stdin`, trying as many times as necessary
fn scanInt() -> i32 {
    loop {
        let scannedString = scanString();

        match scannedString.parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }
}

// Scan a positive nonzero `i32` from `stdin`, trying as many times as necessary
fn scanStrictlyPositiveInt() -> i32 {
    loop {
        let scannedInt = scanInt();
        if scannedInt > 0 {
            return scannedInt;
        }
    }
}

fn main() {
    // Get num holes to play
    println!("How many holes will you play?");
    let numHolesToPlay = scanStrictlyPositiveInt();
    let mut numHolesPlayed: i32 = 0;

    // Get num players who will play
    println!("How many players?");
    let numPlayers = scanStrictlyPositiveInt();

    // Make Vec of Players
    let mut players: Vec<Player> = Vec::new();
    for playerNum in 1..(numPlayers + 1) {
        // Prompt each player for name
        println!("What is player {}'s name?", playerNum);
        let scannedPlayerName = scanString();
        let mut player = Player {
            scores: vec![0],
            name: scannedPlayerName,
            totalScore: 0,
        };
        // Put player in `players` Vec
        players.push(player);
    }

    // Loop for each hole of the game
    while numHolesPlayed < numHolesToPlay {
        numHolesPlayed += 1;

        // Print current standings
        println!();
        println!("Hole {} standings: ", numHolesPlayed);
        players.sort_by(|a, b| b.totalScore.cmp(&a.totalScore));
        for player in players.iter() {
            print!("  {} has {}:  ", player.name, player.totalScore);
            for individualHoleScore in player.scores.iter() {
                print!("{} ", individualHoleScore);
            }
            println!();
        }

        // Ask for results of hole for each player
        println!();
        for mut player in players.iter_mut() {
            println!("What did {} score?", player.name);
            let score = scanInt();
            // Add this hole's score to previous scores
            player.justScored(score);
        }
    }

    // Print final standings
    println!();
    println!("Final standings:");
    players.sort_by(|a, b| b.totalScore.cmp(&a.totalScore));
    for player in players.iter() {
        print!("  {} has {}:  ", player.name, player.totalScore);
        for individualHoleScore in player.scores.iter() {
            print!("{} ", individualHoleScore);
        }
        println!();
    }
}
