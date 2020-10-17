use ttt_engine::{TTTEngine, TTTError, TTTShape};
fn game_over(engine: &TTTEngine) -> bool {
    let winner = engine.check_winner();
    let over = if let Some(winner) = winner {
        println!("The winner is {}", winner);
        true
    } else if engine.complete() {
        println!("It is a tie no winner!");
        true
    } else {
        false
    };
    over
}

fn input(msg: &str) -> std::io::Result<String> {
    let mut s = String::new();
    println!("{}", msg);
    std::io::stdin().read_line(&mut s)?;
    Ok(s.replace("\r\n", ""))
}

fn input_coord(msg: &str, engine: &TTTEngine) -> usize {
    let board_len = engine.get_board().len();
    let parse = move || {
        input(msg)
            .unwrap()
            .parse::<usize>()
            .map_err(|_| "Please enter an integer!".to_string())
            .and_then(|num| {
                if num >= board_len {
                    Err("Integer must be from 0 to 2".to_string())
                } else {
                    Ok(num)
                }
            })
    };

    let mut num = parse();
    while num.is_err() {
        println!("Invalid entry! Reason: {}", num.unwrap_err());
        println!("{}", engine.board_to_string());
        num = parse();
    }
    num.unwrap()
}
fn main() -> Result<(), TTTError> {
    let mut engine = TTTEngine::new();
    for i in TTTShape::X as usize..TTTShape::Blank as usize {
        let name = input(&format!(
            "Enter name for player{}({})",
            i + 1,
            TTTShape::from(i)
        ))
        .unwrap_or_else(|why| {
            println!("Could not get name from stdin! Raw Err: {}", why);
            std::process::exit(-1);
        });
        engine.add_player(&name)?;
    }
    let mut rematch = true;
    while rematch {
        println!("{}", engine.board_to_string());
        while !game_over(&engine) {
            let current_player = engine.get_current_player();
            println!("{}'s Turn!", current_player);
            let x = input_coord("Enter x coordinate", &engine);
            let y = input_coord("Enter y coordinate", &engine);
            if let Err(err) = engine.next_turn(x, y) {
                match err {
                    TTTError::IndexOutOfRange => {
                        println!("({},{}) is not a proper place on the board!", x, y)
                    }
                    TTTError::ShapeAlreadyPlaced => println!("({}, {}) is already taken!", x, y),
                }
            }
            println!("{}", engine.board_to_string());
        }

        let mut rematch_choice = String::new();
        while &rematch_choice != "y" && &rematch_choice != "n" {
            rematch_choice = input("Rematch? y/n").unwrap();
        }

        rematch = rematch_choice == "y";
        if rematch {
            engine.reset_game();
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{size_of, size_of_val};
    use ttt_engine::Player;
    #[test]
    fn mem_sizes() {
        let size = size_of::<TTTShape>();
        println!("Shape size = {}", size);
        let size = size_of_val(&Player::default());
        println!("Default player size = {}", size);
        let size = size_of::<Player>();
        println!("Static player size = {}", size);
        let engine = TTTEngine::new();
        let size = size_of_val(&engine);
        println!("Size of TTTEngine = {}", size);
    }
}
