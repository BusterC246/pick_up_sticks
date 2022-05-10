use std::io::{stdout, Write};
use std::{
    io, 
    thread::sleep, 
    time::Duration,
};

pub fn play_game() {
    println!("Enter the number of sticks (5-50): ");
    let mut sticks = 0;

    while !(5..=50).contains(&sticks) {
        sticks = match get_input().parse::<i8>() {
            Ok(num) => num,
            _ => continue,
        };
    }

    if sticks % 4 == 1 {
        while sticks > 0 {
            sticks = player_turn(sticks);
            display_sticks(sticks);
            sticks = computer_turn(sticks);
            display_sticks(sticks);
        }
    } else {
        while sticks > 0 {
            sticks = computer_turn(sticks);
            display_sticks(sticks);
            sticks = player_turn(sticks);
            display_sticks(sticks);
        }
    }

    println!("You lose.");
}

fn player_turn(sticks: i8) -> i8 {
    println!("\nEnter a number of sticks to remove (1-3): ");

    let mut amount: i8 = 0;

    while !(1..=3).contains(&amount) {
        amount = match get_input().parse::<i8>() {
            Ok(num) => num,
            _ => continue,
        };
    }

    sticks - amount
}

fn computer_turn(sticks: i8) -> i8 {
    for i in 1..=3 {
        if (sticks - i) % 4 == 1 {
            return sticks - i;
        }
    }

    1
}

fn display_sticks(sticks: i8) {
    let mut stdout = stdout();
    let pile = "||||||||||||||||||||||||||||||||||||||||||||||||||";

    for i in 1..=sticks {
        print!("\r{}", &pile[..i as usize]);
        stdout.flush().unwrap();
        sleep(Duration::from_millis(20));
    }

    println!();
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Trouble reading input.");

    input.trim().to_string()
}