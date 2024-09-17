/*
Name: Jon Bennett
Date: 03/10/2024
Class: CISS 301

Discription:
This program uses two FSM (Finite State Machines) to run a game where you can move around a character in a grid with boarder.
If you jump twice while moving the game will cause you to fall in which ever direction your character is moving, which will
cause them to die. you will be given an option to run the program again if you want. This will go on for ever until you jump
twice. 
*/
use std::io::{self, Write};

#[derive(Clone, Debug, PartialEq)]
//This is state for the First FSM.
enum State {
    Laying,
    Sitting,
    Standing,
    Walking(Direction),
    Running(Direction),
    Jumping(Direction),
    Falling(Direction),
    Dead,
    WaitingForDirection(MotionType),
}

#[derive(Clone, Debug, PartialEq)]
enum MotionType {
    Walking,
    Running,
}

#[derive(Clone, Debug, PartialEq)]
//This is the State for the second FSM
enum Direction {
    Forward,
    Backward,
    Left,
    Right,
    Up,
}

//implmentation for the 1st FSM combinded with the 2nd FSM for direction.
//This is for the 1st FSM printed output. 
impl State {
    fn display(&self) -> String {
        match self {
            State::Laying => "Character is laying down.".to_string(),
            State::Sitting => "Character is sitting.".to_string(),
            State::Standing => "Character is standing.".to_string(),
            State::Walking(direction) => {
                format!("Character is walking {}", direction.display())
            }
            State::Running(direction) => {
                format!("Character is running {}", direction.display())
            }
            State::Jumping(direction) => {
                format!("Character is jumping {}", direction.display())
            }
            State::Falling(direction) => {
                format!("Character is falling {}", direction.display())
            }
            State::Dead => "☠️ ".to_string(),
            State::WaitingForDirection(motion_type) => match motion_type {
                MotionType::Walking => "Waiting for walking direction input.".to_string(),
                MotionType::Running => "Waiting for running direction input.".to_string(),
            },
        }
    }
//This is the actual FSM that is both put together.
//This will provide a State to State transfer for
//both the State FSM and Direction FSM.  
    fn transition(&self, input: &str) -> State {
        match (self, input) {
            (State::Laying, "h") => State::Sitting,
            (State::Sitting, "h") => State::Standing,
            (State::Standing, "l") => State::Sitting,
            (State::Sitting, "l") => State::Laying, 
            (State::Walking(Direction::Forward), "l") => State::Standing,
            (State::Walking(Direction::Backward), "l") => State::Standing,
            (State::Walking(Direction::Left), "l") => State::Standing,
            (State::Walking(Direction::Right), "l") => State::Standing,
            (State::Running(Direction::Forward), "l") => State::Standing,
            (State::Running(Direction::Backward), "l") => State::Standing,
            (State::Running(Direction::Left), "l") => State::Standing,
            (State::Running(Direction::Right), "l") => State::Standing,
            (State::Standing, "k") => State::WaitingForDirection(MotionType::Walking),
            (State::Walking(Direction::Forward), "k") => State::WaitingForDirection(MotionType::Running),
            (State::Walking(Direction::Backward), "k") => State::WaitingForDirection(MotionType::Running),
            (State::Walking(Direction::Left), "k") => State::WaitingForDirection(MotionType::Running),
            (State::Walking(Direction::Right), "k") => State::WaitingForDirection(MotionType::Running),
            (State::Running(Direction::Forward), "k") => State::WaitingForDirection(MotionType::Walking),
            (State::Running(Direction::Backward), "k") => State::WaitingForDirection(MotionType::Walking),
            (State::Running(Direction::Left), "k") => State::WaitingForDirection(MotionType::Walking),
            (State::Running(Direction::Right), "k") => State::WaitingForDirection(MotionType::Walking),
            (State::Standing, "j") => State::Jumping(Direction::Up),
            (State::Jumping(Direction::Up), _) => State::Standing,
            (State::Walking(Direction::Forward), "j") => State::Jumping(Direction::Forward),
            (State::Walking(Direction::Backward), "j") => State::Jumping(Direction::Backward),
            (State::Walking(Direction::Left), "j") => State::Jumping(Direction::Left),
            (State::Walking(Direction::Right), "j") => State::Jumping(Direction::Right),
            (State::Running(Direction::Forward), "j") => State::Jumping(Direction::Forward),
            (State::Running(Direction::Backward), "j") => State::Jumping(Direction::Backward),
            (State::Running(Direction::Left), "j") => State::Jumping(Direction::Left),
            (State::Running(Direction::Right), "j") => State::Jumping(Direction::Right),
            (State::Jumping(Direction::Forward), "!j") => State::Walking(Direction::Forward),
            (State::Jumping(Direction::Forward), "j") => State::Falling(Direction::Forward),
            (State::Jumping(Direction::Forward), _) => State::Walking(Direction::Forward),
            (State::Jumping(Direction::Backward), "!j") => State::Walking(Direction::Backward),
            (State::Jumping(Direction::Backward), "j") => State::Falling(Direction::Backward),
            (State::Jumping(Direction::Backward), _) => State::Walking(Direction::Backward),
            (State::Jumping(Direction::Left), "!j") => State::Walking(Direction::Left),
            (State::Jumping(Direction::Left), "j") => State::Falling(Direction::Left),
            (State::Jumping(Direction::Left), _) => State::Walking(Direction::Left),
            (State::Jumping(Direction::Right), "!j") => State::Walking(Direction::Right),
            (State::Jumping(Direction::Right), "j") => State::Falling(Direction::Right),
            (State::Jumping(Direction::Right), _) => State::Walking(Direction::Right),
            (State::Falling(Direction::Forward), _) => State::Dead,
            (State::Falling(Direction::Backward), _) => State::Dead,
            (State::Falling(Direction::Left), _) => State::Dead,
            (State::Falling(Direction::Right), _) => State::Dead,
            (State::Standing, "w") => State::Walking(Direction::Forward),
            (State::Standing, "s") => State::Walking(Direction::Backward),
            (State::Standing, "a") => State::Walking(Direction::Left),
            (State::Standing, "d") => State::Walking(Direction::Right),
            (State::Walking(Direction::Backward), "w") => State::Walking(Direction::Forward),
            (State::Walking(Direction::Left), "w") => State::Walking(Direction::Forward),
            (State::Walking(Direction::Right), "w") => State::Walking(Direction::Forward),
            (State::Running(Direction::Backward), "w") => State::Running(Direction::Forward),
            (State::Running(Direction::Left), "w") => State::Running(Direction::Forward),
            (State::Running(Direction::Right), "w") => State::Running(Direction::Forward),
            (State::Walking(Direction::Forward), "s") => State::Walking(Direction::Backward),
            (State::Walking(Direction::Left), "s") => State::Walking(Direction::Backward),
            (State::Walking(Direction::Right), "s") => State::Walking(Direction::Backward),
            (State::Running(Direction::Forward), "s") => State::Running(Direction::Backward),
            (State::Running(Direction::Left), "s") => State::Running(Direction::Backward),
            (State::Running(Direction::Right), "s") => State::Running(Direction::Backward),
            (State::Walking(Direction::Backward), "a") => State::Walking(Direction::Left),
            (State::Walking(Direction::Forward), "a") => State::Walking(Direction::Left),
            (State::Walking(Direction::Right), "a") => State::Walking(Direction::Left),
            (State::Running(Direction::Backward), "a") => State::Running(Direction::Left),
            (State::Running(Direction::Forward), "a") => State::Running(Direction::Left),
            (State::Running(Direction::Right), "a") => State::Running(Direction::Left),
            (State::Walking(Direction::Backward), "d") => State::Walking(Direction::Right),
            (State::Walking(Direction::Left), "d") => State::Walking(Direction::Right),
            (State::Walking(Direction::Forward), "d") => State::Walking(Direction::Right),
            (State::Running(Direction::Backward), "d") => State::Running(Direction::Right),
            (State::Running(Direction::Left), "d") => State::Running(Direction::Right),
            (State::Running(Direction::Forward), "d") => State::Running(Direction::Right),
            (State::WaitingForDirection(MotionType::Walking), "w") => {
                State::Walking(Direction::Forward)
            }
            (State::WaitingForDirection(MotionType::Walking), "s") => {
                State::Walking(Direction::Backward)
            }
            (State::WaitingForDirection(MotionType::Walking), "a") => {
                State::Walking(Direction::Left)
            }
            (State::WaitingForDirection(MotionType::Walking), "d") => {
                State::Walking(Direction::Right)
            }
            (State::WaitingForDirection(MotionType::Running), "w") => {
                State::Running(Direction::Forward)
            }
            (State::WaitingForDirection(MotionType::Running), "s") => {
                State::Running(Direction::Backward)
            }
            (State::WaitingForDirection(MotionType::Running), "a") => {
                State::Running(Direction::Left)
            }
            (State::WaitingForDirection(MotionType::Running), "d") => {
                State::Running(Direction::Right)
            }            
            _ => self.clone(),
        }
    }
}

//The printed output from the 2nd FSM. 
impl Direction {
    fn display(&self) -> &'static str {
        match self {
            Direction::Forward => "forward",
            Direction::Backward => "backward",
            Direction::Left => "left",
            Direction::Right => "right",
            Direction::Up => "Up",
        }
    }


//This function provides the information for movement of the character. While checking for the boundaries. 
//That way the character doesn't move off grid. 
    fn move_position(&self, position: (usize, usize), steps: usize, grid_size: usize) -> (usize, usize) {
        let mut new_position = position;
        match self {
            Direction::Forward => new_position.0 = new_position.0.saturating_sub(steps),
            Direction::Backward => new_position.0 = new_position.0.saturating_add(steps),
            Direction::Left => new_position.1 = new_position.1.saturating_sub(steps),
            Direction::Right => new_position.1 = new_position.1.saturating_add(steps),
            _ => (),
        }
        
        // Check if new position is within grid boundaries
        new_position.0 = new_position.0.min(grid_size - 1); // Ensure row is within grid bounds
        new_position.1 = new_position.1.min(grid_size - 1); // Ensure column is within grid bounds

        new_position
    }
}

//This function prints a visual for the user so that they can see where they are at on the grid. 
fn display_grid(character_position: (usize, usize), character_state: &State, grid_size: usize) {
    // Print top border
    println!("+{}+", "-".repeat(grid_size * 2));

    for i in 0..grid_size {
        print!("|");
        for j in 0..grid_size {
            if (i, j) == character_position {
                match character_state {
                    State::Laying => print!("💤 "),
                    State::Sitting => print!("🪑 "),
                    State::Standing => print!("🕴️ "),
                    State::Walking(_) => print!("🚶 "),
                    State::Running(_) => print!("🏃 "),
                    State::Jumping(_) => print!("🏃🌠 "),
                    State::Falling(_) => print!("🌠 "),
                    State::Dead => print!("☠️ "),
                    State::WaitingForDirection(_) => print!("⏳ "),
                }
            } else {
                print!(". "); // Print empty space at other positions
            }
        }
        println!("|"); // End of row
    }

    // Print bottom border
    println!("+{}+", "-".repeat(grid_size * 2));
}


fn main() {
    let mut play_again = true;

    while play_again {
        let mut current_state = State::Laying;
        let mut character_position = (8, 7); // Initial character position
        let grid_size = 15; // Size of the grid

        println!("Enter the state string (h, j, k, l, w, a, s, d) to control the character
    - h is to stand up. 
    - l is to stop or lay down
    - k is to toggle walking/running
    - j is to jump, be careful not to jump twice while moving. 
    - w is to move forward
    - a is to move left
    - s is to move backwards
    - d is to move right");

        loop {
            println!("{}", current_state.display());
            display_grid(character_position, &current_state, grid_size);

            let mut input = String::new();
            io::stdout().flush().expect("Failed to flush stdout");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim().to_lowercase();

            if input.is_empty() {
                continue;
            }

            if let State::Walking(ref direction) | State::Running(ref direction) | State::Jumping(ref direction) = current_state {
                let steps = match current_state {
                    State::Walking(_) => 1,
                    State::Running(_) => 2,
                    State::Jumping(_) => 3,
                    _ => 0,
                };

                character_position = direction.move_position(character_position, steps, grid_size);
            }

            current_state = current_state.transition(&input);

            if current_state == State::Dead {
                println!("{}", current_state.display());
                println!("Character is dead. Game over.");
                break;
            }
        }

        // Ask if player wants to play again
        loop {
            println!("Do you want to play again? (yes/no)");
        
            let mut play_input = String::new();
            io::stdout().flush().expect("Failed to flush stdout");
            io::stdin()
                .read_line(&mut play_input)
                .expect("Failed to read line");
            let play_input = play_input.trim().to_lowercase();
        
            match play_input.as_str() {
                "yes" => {
                    play_again = true;
                    break;
                }
                "no" => {
                    play_again = false;
                    break;
                }
                _ => {
                    println!("Invalid input. Please enter 'yes' or 'no'.");
                }
            }
        }        
    }
} 

