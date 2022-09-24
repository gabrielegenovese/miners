use rand::Rng;
use std::io;
use std::process::exit;

struct GridT {
    x: i32,
    y: i32,
    mines: i32,
    field: Vec<i32>,
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

    let grid: Vec<i32> = init_grid(mode as u8);
    let are_ya_winning_son = game_loop(grid);

    if are_ya_winning_son {
        println!("Hai vinto :>");
    } else {
        println!("Hai perso :(");
    }
}

fn print_grid(grid: &Vec<i32>) {
    println!();
    print!("  ");
    for i in 0..9 {
        print!("{} ", i);
    }
    for i in 0..grid.len() {
        if i % 9 == 0 {
            println!();
            print!("{} ", (i % 9));
        }
        if grid[i] == -1 {
            print!("| ");
        } else {
            print!("{} ", grid[i]);
        }
    }
    println!("\n");
}

// per semplicità uso un vettore unidimensionale per accedere vec[y][x] -> vec[y*2+x]
fn init_grid(mode: u8) -> Vec<i32> {
    let x;
    let y;
    let mut mines;

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
    let mut grid: Vec<i32> = Vec::new();
    for _ in 0..x {
        for _ in 0..y {
            grid.push(0);
        }
    }

    //pizzo le mine
    while mines != 0 {
        let tmpx: i32 = rng.gen_range(0..x);
        let tmpy: i32 = rng.gen_range(0..y);
        if grid[(tmpy as usize)* (x as usize) + (tmpx as usize)] != -1 {
            grid[tmpy as usize * x as usize + tmpx as usize] = -1;
            mines -= 1;
            
            if (tmpy - 1) * x + (tmpx - 1) > 0 && (tmpy - 1) * x + (tmpx - 1) < grid.len() as i32 {
                grid[((tmpy - 1) as usize * x as usize) + (tmpx - 1) as usize] += 1;
            }
            if ((tmpy - 1) * x) + tmpx > 0 && ((tmpy - 1) * x) + tmpx < grid.len() as i32 {
                grid[((tmpy - 1) as usize * x as usize) + tmpx as usize] += 1;
            }
            if (tmpy - 1) * x + (tmpx + 1) > 0 && (tmpy - 1) * x + (tmpx + 1) < grid.len() as i32 {
                grid[((tmpy - 1) as usize * x as usize) + (tmpx + 1) as usize] += 1;
            }
            if tmpy * x + (tmpx - 1) > 0 && tmpy * x + (tmpx - 1) < grid.len() as i32 {
                grid[(tmpy * x) as usize + (tmpx - 1) as usize] += 1;
            }
            if tmpy * x + (tmpx + 1) > 0 && tmpy * x + (tmpx + 1) < grid.len() as i32 {
                grid[(tmpy * x) as usize + (tmpx + 1) as usize] += 1;
            }
            if (tmpy + 1) * x + (tmpx - 1) > 0 && (tmpy + 1) * x + (tmpx - 1) < grid.len() as i32 {
                grid[((tmpy + 1) as usize * x as usize) + (tmpx - 1) as usize] += 1;
            }
            if ((tmpy + 1) * x) + tmpx > 0 && ((tmpy + 1) * x) + tmpx < grid.len() as i32 {
                grid[((tmpy + 1) as usize * x as usize) + tmpx as usize] += 1;
            }
            if (tmpy + 1) * x + (tmpx + 1) > 0 && (tmpy + 1) * x + (tmpx + 1) < grid.len() as i32 {
                grid[((tmpy + 1) as usize * x as usize) + (tmpx + 1) as usize] += 1;
            }
        }
    }

    grid
}
fn game_loop(grid: Vec<i32>) -> bool {
    let mut mode = String::new();
    loop {
        print_grid(&grid);
        io::stdin().read_line(&mut mode).expect("Errore :(");

        return true;
    }
}
