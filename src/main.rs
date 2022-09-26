use colored::Colorize;
use rand::Rng;
use std::io;
use std::process::exit;

pub struct FieldInfo {
    x: i32,
    y: i32,
    mines: i32,
    field: Vec<i32>,
    flagged: Vec<bool>,
    visible: Vec<bool>,
}

fn main() {
    println!(
        "Welcome to Miners, a minesweeper-like for terminal, written in Rust!
Choose a difficulty:
    1 - Noob
    2 - Intermediate
    3 - Expert
"
    );

    let mut info: FieldInfo = init_grid(get_difficulty() as u8);

    println!("
Instruction to play:
  f - flag/unflag a cell
  v - view a cell
followed by y,x. For example f3,1 or c4,8 are valid command.
    ");

    let are_ya_winning_son = game_loop(&mut info);

    if are_ya_winning_son {
        println!("{}", "You won! :>".green());
    } else {
        println!("{}", "Booom, you lost :(".red());
    }
}

fn get_difficulty() -> i8 {
    let mut mode = String::new();
    io::stdin().read_line(&mut mode).expect("Errore :(");
    let mode: i8 = match mode.trim().parse() {
        Ok(num) => num,
        Err(_) => -1,
    };
    assert!(mode > 1 || mode < 4);
    mode
}

fn print_grid(info: &FieldInfo) {
    print!("\n  ");
    // print first row of numbers
    for i in 0..info.x {
        print!("{} ", i);
    }
    for i in 0..info.field.len() {
        // print column number
        if i as i32 % info.x == 0 {
            print!("\n{} ", i as i32 / info.y);
        }

        // print cell
        if info.visible[i] {
            print_number(info.field[i]);
        } else if info.flagged[i] {
            print!("{} ", "f".red());
        } else {
            print!("{}", "  ".on_truecolor(164, 164, 164)); // print gray bg
        }
    }
    println!("\n");
}

// print bomb or number with colors
fn print_number(num: i32) {
    match num {
        -1 => print!("{} ", "o".bright_red()),
        0 => print!("  "),
        1 => print!("{} ", "1".blue()),
        2 => print!("{} ", "2".green()),
        3 => print!("{} ", "3".red()),
        4 => print!("{} ", "4".color("yellow")),
        5 => print!("{} ", "5".color("brown")),
        6 => print!("{} ", "6".color("cyan")),
        7 => print!("{} ", "7".bright_magenta()),
        8 => print!("{} ", "8".purple()),
        9 => print!("{} ", "8".bright_green()),
        _ => print!("Errore in print_number"),
    };
}

// the fn will init a modo dimentional vector, vec[row][col] -> vec[row*y+col]
fn init_grid(mode: u8) -> FieldInfo {
    let x;
    let y;
    let mines;

    let mut rng = rand::thread_rng();

    match mode {
        // noob
        1 => {
            x = 9;
            y = 9;
            mines = 10;
        }
        // intermediete
        2 => {
            x = 16;
            y = 16;
            mines = 40;
        }
        // expert
        3 => {
            x = 30;
            y = 16;
            mines = 99;
        }
        _ => exit(1),
    }

    // init vectors
    let mut field: Vec<i32> = Vec::new();
    let mut flagged: Vec<bool> = Vec::new();
    let mut visible: Vec<bool> = Vec::new();
    for _ in 0..x {
        for _ in 0..y {
            field.push(0);
            flagged.push(false);
            visible.push(false);
        }
    }

    //set mines casually
    let mut c = mines;
    while c != 0 {
        let tmpx: i32 = rng.gen_range(0..x);
        let tmpy: i32 = rng.gen_range(0..y);
        if field[(tmpy * x + tmpx) as usize] != -1 {
            field[(tmpy * x + tmpx) as usize] = -1;
            c -= 1;

            // add 1 around the mine selected
            if tmpy - 1 >= 0 {
                if tmpx - 1 >= 0 && field[((tmpy - 1) * x + tmpx - 1) as usize] != -1 {
                    field[((tmpy - 1) * x + tmpx - 1) as usize] += 1;
                }
                if field[((tmpy - 1) * x + tmpx) as usize] >= 0 {
                    field[((tmpy - 1) * x + tmpx) as usize] += 1;
                }
                if tmpx + 1 < x && field[((tmpy - 1) * x + tmpx + 1) as usize] != -1 {
                    field[((tmpy - 1) * x + tmpx + 1) as usize] += 1;
                }
            }

            if tmpx - 1 >= 0 && field[(tmpy * x + tmpx - 1) as usize] != -1 {
                field[(tmpy * x + tmpx - 1) as usize] += 1;
            }
            if tmpx + 1 < x && field[(tmpy * x + tmpx + 1) as usize] != -1 {
                field[(tmpy * x + tmpx + 1) as usize] += 1;
            }

            if tmpy + 1 < y {
                if tmpx - 1 >= 0 && field[((tmpy + 1) * x + tmpx - 1) as usize] != -1 {
                    field[((tmpy + 1) * x + tmpx - 1) as usize] += 1;
                }
                if field[((tmpy + 1) * x + tmpx) as usize] != -1 {
                    field[((tmpy + 1) * x + tmpx) as usize] += 1;
                }
                if tmpx + 1 < x && field[((tmpy + 1) * x + tmpx + 1) as usize] != -1 {
                    field[((tmpy + 1) * x + tmpx + 1) as usize] += 1;
                }
            }
        }
    }

    FieldInfo{x,y,mines,field,flagged,visible,}
}

fn game_loop(info: &mut FieldInfo) -> bool {
    loop {
        let mut user_input = String::new();
        print_grid(&info);

        io::stdin().read_line(&mut user_input).expect("Errore :(");

        // get the user command
        let command = user_input.chars().next().unwrap();
        let y = user_input.chars().nth(1).unwrap().to_digit(10).unwrap() as i32;
        let x = user_input.chars().nth(3).unwrap().to_digit(10).unwrap() as i32;

        if command == 'v' {
            info.visible[(y * info.y + x) as usize] = true;
        }
        if command == 'f' {
            if info.flagged[(y * info.y + x) as usize] {
                info.flagged[(y * info.y + x) as usize] = false;
            } else {
                info.flagged[(y * info.y + x) as usize] = true;
            }
        }

        // check loss
        for i in 0..info.mines {
            if info.field[(y * info.y + x) as usize] == -1 && info.visible[i as usize] {
                print_grid(&info);
                return false;
            }
        }

        // check win
        let mut c = 0;
        for i in 0..info.field.len() {
            if info.visible[i] {
                c += 1;
            }
        }
        if c == info.field.len() - 10 {
            return true;
        }
    }
}
