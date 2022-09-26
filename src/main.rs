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
        "Benvenuto in Miners, un copia di campo minato da cui 
puoi giocare da terminale, creato intermanente in Rust!
Scegli la modalità di gioco:
    1 - Principiante
    2 - Intermedio
    3 - Esperto
"
    );

    let mut mode = String::new();

    io::stdin().read_line(&mut mode).expect("Errore :(");
    let mode: i8 = match mode.trim().parse() {
        Ok(num) => num,
        Err(_) => -1,
    };

    assert!(mode > 1 || mode < 4);

    let mut info: FieldInfo = init_grid(mode as u8);
    let are_ya_winning_son = game_loop(&mut info);

    if are_ya_winning_son {
        println!("Hai vinto :>");
    } else {
        println!("Hai perso :(");
    }
}

fn print_grid(info: &FieldInfo) {
    print!("\n  ");
    for i in 0..info.x {
        print!("{} ", i);
    }
    for i in 0..info.field.len() {
        // stampa del numero
        if i as i32 % info.x == 0 {
            print!("\n{} ", i as i32 / info.y);
        }

        // stampa della cella
        if info.visible[i] {
            if info.field[i] == -1 {
                print!("{} ", "o".bright_red()); // stampo la bomba
            } else {
                if info.field[i as usize] == 0 {
                    print!("  "); // stampo bianco se è 0
                } else {
                    if info.field[i as usize] == 1 {
                        print!("{} ", "1".blue()); // stampo il numero
                    }
                    if info.field[i as usize] == 2 {
                        print!("{} ", "2".green()); // stampo il numero
                    }
                    if info.field[i as usize] == 3 {
                        print!("{} ", "3".red()); // stampo il numero
                    }
                    if info.field[i as usize] == 4 {
                        print!("{} ", "4".color("yellow")); // stampo il numero
                    }
                    if info.field[i as usize] == 5 {
                        print!("{} ", "5".color("brown")); // stampo il numero
                    }
                    if info.field[i as usize] == 6 {
                        print!("{} ", "6".color("cyan")); // stampo il numero
                    }
                    if info.field[i as usize] == 7 {
                        print!("{} ", "6".color("gray")); // stampo il numero
                    }
                }
            }
        } else if info.flagged[i] {
            print!("{} ", "f".red()); // stampo la flag
        } else {
            print!("{}", "  ".on_truecolor(164, 164, 164)); // stampo bianco se è 0
        }
    }
    println!("\n");
}

// per semplicità uso un vettore unidimensionale per accedere vec[row][col] -> vec[row*y+col]
fn init_grid(mode: u8) -> FieldInfo {
    let x;
    let y;
    let mines;

    let mut rng = rand::thread_rng();

    match mode {
        1 => {
            x = 9;
            y = 9;
            mines = 10;
        }
        2 => {
            x = 16;
            y = 16;
            mines = 40;
        }
        3 => {
            x = 30;
            y = 16;
            mines = 99;
        }
        _ => exit(1),
    }

    // inizializzo il vettore
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

    //pizzo le mine
    let mut c = mines;
    while c != 0 {
        let tmpx: i32 = rng.gen_range(0..x);
        let tmpy: i32 = rng.gen_range(0..y);
        if field[(tmpy * x + tmpx) as usize] != -1 {
            field[(tmpy * x + tmpx) as usize] = -1;
            c -= 1;

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

    FieldInfo {
        x,
        y,
        mines,
        field,
        flagged,
        visible,
    }
}

fn game_loop(info: &mut FieldInfo) -> bool {
    loop {
        let mut user_input = String::new();
        print_grid(&info);

        io::stdin().read_line(&mut user_input).expect("Errore :(");

        // elabora il comando dell'utente
        let command = user_input.chars().next().unwrap();
        let y = user_input.chars().nth(1).unwrap().to_digit(10).unwrap() as i32;
        let x = user_input.chars().nth(3).unwrap().to_digit(10).unwrap() as i32;

        if command == 'c' {
            info.visible[(y * info.y + x) as usize] = true;
        }
        if command == 'f' {
            info.flagged[(y * info.y + x) as usize] = true;
        }

        // controllo per la perdità
        for i in 0..info.mines {
            if info.field[(y * info.y + x) as usize] == -1 && info.visible[i as usize] {
                print_grid(&info);
                return false; // u lost the game
            }
        }

        // controllo per la vincita
        let mut c = 0;
        for i in 0..info.field.len() {
            if info.visible[i] {
                c += 1;
            }
        }
        if c == info.visible.len() - 10 {
            return true; // vinci se tutte le celle sono visibili tranne le mine
        }
    }
}
