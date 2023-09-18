use clearscreen;
use itertools::Itertools;
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

struct LifeGame {
    H: usize,
    W: usize,
    screen: Vec<Vec<char>>,
}

impl LifeGame {
    fn new(H: usize, W: usize) -> LifeGame {
        let screen = vec![vec![' '; W]; H];
        LifeGame { screen, H, W }
    }

    fn init_random(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.H {
            for j in 0..self.W {
                if rng.gen_range(0..=1) == 1 {
                    self.screen[i][j] = 'o';
                }
            }
        }
    }

    fn show(&self) {
        for v in &self.screen {
            println!("{}", v.iter().join(""));
        }
    }

    fn count(&self, x: usize, y: usize) -> usize {
        let dx = [1, 0, -1, 0, 1, 1, -1, -1];
        let dy = [0, 1, 0, -1, 1, -1, 1, -1];
        let mut cnt = 0;
        for i in 0..8 {
            let nx = x as isize + dx[i];
            let ny = y as isize + dy[i];
            if !(0 <= nx && nx < self.H as isize && 0 <= ny && ny < self.W as isize) {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if self.screen[nx][ny] == 'o' {
                cnt += 1;
            }
        }
        cnt
    }

    fn update(&mut self) {
        let mut new_screen = self.screen.clone();
        for i in 0..self.H {
            for j in 0..self.W {
                let cnt = self.count(i, j);
                if self.screen[i][j] == ' ' {
                    if cnt == 3 {
                        new_screen[i][j] = 'o';
                    }
                } else {
                    if cnt == 2 || cnt == 3 {
                        // Save
                    } else {
                        new_screen[i][j] = ' ';
                    }
                }
            }
        }
        self.screen = new_screen;
    }
}

fn main() {
    let H: usize = 40;
    let W: usize = 100;
    let mut game = LifeGame::new(H, W);
    // let penta = vec![
    //     "oooooooo".chars().collect_vec(),
    //     "o oooo o".chars().collect_vec(),
    //     "oooooooo".chars().collect_vec(),
    // ];
    // for i in 0..3 {
    //     for j in 0..8 {
    //         game.screen[i + 10][j + 20] = penta[i][j] as char;
    //     }
    // }
    game.init_random();
    clearscreen::clear().unwrap();
    let mut gen = 0;
    println!("generation: {}", gen);
    game.show();
    sleep(Duration::from_secs(3));
    loop {
        clearscreen::clear().unwrap();
        println!("generation: {}", gen);
        game.show();
        game.update();
        sleep(Duration::from_millis(50));
        gen += 1;
    }
}
