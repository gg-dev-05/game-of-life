// mod screen;
// mod state;
use rand::Rng;
use std::{thread, time::Duration};
const WIDTH: usize = 50;
const HEIGHT: usize = 10;

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

fn sleep(time: u64) {
    thread::sleep(Duration::from_millis(time));
}

fn draw(x: u8) {
    if x == 1 {
        print!("\x1b[92m{} \x1b[0m", "X");
    } else {
        print!("\x1b[93m{} \x1b[0m", "O");
    }
}

fn randomize(array: &mut [[u8; WIDTH]; HEIGHT]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let num = rand::thread_rng().gen_range(0..2);
            // println!("{}")
            if num == 1 {
                array[y][x] = 1;
                print!(".");
            } else {
                array[y][x] = 0;
            }
        }
    }
}

fn disp(array: [[u8; WIDTH]; HEIGHT], info: String) {
    clear();
    println!("{}", info);
    for y in array {
        for x in y {
            draw(x);
        }
        println!("");
    }
    sleep(100);
}

fn simulate(front_screen: &mut [[u8; WIDTH]; HEIGHT]) {
    // let y_direction: [i8; 4] = [0, -1, 0, 1];
    // let x_direction: [i8; 4] = [-1, 0, 1, 0];
    let temp: [[u8; WIDTH]; HEIGHT];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut alive = 0;
            if y != 0 && front_screen[y - 1][x] == 1 {
                alive += 1;
            }
            if y != HEIGHT - 1 && front_screen[y + 1][x] == 1 {
                alive += 1;
            }
            if x != 0 && front_screen[y][x - 1] == 1 {
                alive += 1;
            }
            if x != WIDTH - 1 && front_screen[y][x + 1] == 1 {
                alive += 1;
            }
            // for i in 0..4 {
            //     if 0 <= y as i8 + y_direction[i]
            //         && y as i8 + y_direction[i] < HEIGHT as i8
            //         && 0 <= x as i8 + x_direction[i]
            //         && x as i8 + x_direction[i] < WIDTH as i8
            //     {
            //         if array[y + y_direction[i] as usize][x + x_direction[i] as usize] == 1 {
            //             alive += 1;
            //         }
            //     }
            // }
            if alive < 2 {
                temp[y][x] = 0; // Any live cell with fewer than two live neighbours dies,
            }
            if alive == 2 || alive == 3 {
                temp[y][x] = 1; // Any live cell with two or three live neighbours lives on to the next generation.
            }
            if alive > 3 {
                temp[y][x] = 0; // Any live cell with more than three live neighbours dies
            }
        }
    }
    // front_screen = temp;
}

fn main() {
    clear();
    let mut front_screen = [[0u8; WIDTH]; HEIGHT];
    let mut generation = 1;

    randomize(&mut front_screen);
    disp(front_screen, format!("Generation: 0"));
    while generation < 100 {
        simulate(&mut front_screen);
        disp(front_screen, format!("Genration: {generation}"));
        generation += 1;
    }
}
