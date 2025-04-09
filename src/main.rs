use colored::{ColoredString, Colorize};
use rand::seq::IndexedRandom;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone)]
struct Layout {
    name: String,
    lp: [char; 3],
    lr: [char; 3],
    lm: [char; 3],
    li: [char; 6],
    ri: [char; 6],
    rm: [char; 3],
    rr: [char; 3],
    rp: [char; 3],
}

impl Layout {
    fn from_string(name: &str, columns: &str) -> Layout {
        let mut columnchars = columns.chars();
        Layout {
            name: name.to_string(),
            lp: [
                columnchars.next().unwrap(),
                columnchars.next().unwrap(),
                columnchars.next().unwrap(),
            ],
            lr: [
                columnchars.next().unwrap(),
                columnchars.next().unwrap(),
                columnchars.next().unwrap(),
            ],
            lm: [
                columnchars.next().unwrap(),
                columnchars.next().unwrap(),
                columnchars.next().unwrap(),
            ],
            li: [
                columnchars.next().unwrap(),
                columnchars.next().unwrap(),
                columnchars.next().unwrap(),
                columnchars.next().unwrap(),
                columnchars.next().unwrap(),
                columnchars.next().unwrap(),
            ],
            ri: [
                columnchars.next().unwrap(),
                columnchars.next().unwrap(),
                columnchars.next().unwrap(),
                columnchars.next().unwrap(),
                columnchars.next().unwrap(),
                columnchars.next().unwrap(),
            ],
            rm: [
                columnchars.next().unwrap(),
                columnchars.next().unwrap(),
                columnchars.next().unwrap(),
            ],
            rr: [
                columnchars.next().unwrap(),
                columnchars.next().unwrap(),
                columnchars.next().unwrap(),
            ],
            rp: [
                columnchars.next().unwrap(),
                columnchars.next().unwrap(),
                columnchars.next().unwrap(),
            ],
        }
    }

    fn determine_colour(try_char: char, try_idx: usize, try_column: &[char]) -> ColoredString {
        let mut try_char_idx = 44;
        for (idx, ch) in try_column.iter().enumerate() {
            if *ch == try_char {
                try_char_idx = idx;
            }
        }
        if try_char_idx == 44 {
            try_char.to_string().white()
        } else {
            if try_char_idx == try_idx {
                try_char.to_string().green()
            } else {
                try_char.to_string().yellow()
            }
        }
    }

    fn cmp_and_output(&self, layout: &Layout) -> String {
        format!(
            "{} {} {} {} {}  {} {} {} {} {}\n\
             {} {} {} {} {}  {} {} {} {} {}\n\
             {} {} {} {} {}  {} {} {} {} {}",
            Layout::determine_colour(layout.lp[0], 0, &self.lp),
            Layout::determine_colour(layout.lr[0], 0, &self.lr),
            Layout::determine_colour(layout.lm[0], 0, &self.lm),
            Layout::determine_colour(layout.li[0], 0, &self.li),
            Layout::determine_colour(layout.li[3], 3, &self.li),
            Layout::determine_colour(layout.ri[3], 3, &self.ri),
            Layout::determine_colour(layout.ri[0], 0, &self.ri),
            Layout::determine_colour(layout.rm[0], 0, &self.rm),
            Layout::determine_colour(layout.rr[0], 0, &self.rr),
            Layout::determine_colour(layout.rp[0], 0, &self.rp),
            Layout::determine_colour(layout.lp[1], 1, &self.lp),
            Layout::determine_colour(layout.lr[1], 1, &self.lr),
            Layout::determine_colour(layout.lm[1], 1, &self.lm),
            Layout::determine_colour(layout.li[1], 1, &self.li),
            Layout::determine_colour(layout.li[4], 4, &self.li),
            Layout::determine_colour(layout.ri[4], 4, &self.ri),
            Layout::determine_colour(layout.ri[1], 1, &self.ri),
            Layout::determine_colour(layout.rm[1], 1, &self.rm),
            Layout::determine_colour(layout.rr[1], 1, &self.rr),
            Layout::determine_colour(layout.rp[1], 1, &self.rp),
            Layout::determine_colour(layout.lp[2], 2, &self.lp),
            Layout::determine_colour(layout.lr[2], 2, &self.lr),
            Layout::determine_colour(layout.lm[2], 2, &self.lm),
            Layout::determine_colour(layout.li[2], 2, &self.li),
            Layout::determine_colour(layout.li[5], 5, &self.li),
            Layout::determine_colour(layout.ri[5], 5, &self.ri),
            Layout::determine_colour(layout.ri[2], 2, &self.ri),
            Layout::determine_colour(layout.rm[2], 2, &self.rm),
            Layout::determine_colour(layout.rr[2], 2, &self.rr),
            Layout::determine_colour(layout.rp[2], 2, &self.rp),
        )
    }
}

fn main() {
    let path = Path::new("./src/layouts.txt");
    let mut layoutfile = File::open(path).unwrap();
    let mut layoutsbuf = String::new();
    _ = layoutfile.read_to_string(&mut layoutsbuf);
    let layouts: Vec<Layout> = layoutsbuf
        .split("\r\n\r\n")
        .map(|s| s.trim())
        .map(|s| {
            let mut sp = s.split("\r\n");
            Layout::from_string(sp.next().unwrap(), sp.next().unwrap())
        })
        .collect();

    let mut rng = rand::rng();
    let target = layouts.choose(&mut rng).unwrap().clone();
    let mut guesscount = 0;
    'gameloop: loop {
        println!("\nGuess ({}): ", guesscount);
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        guess = guess.trim().to_string();
        let guessed_layout = match layouts.iter().find(|f| f.name == guess) {
            Some(s) => s,
            None => continue 'gameloop,
        };
        println!("{}", target.cmp_and_output(guessed_layout));
        if target.name == guess {
            println!("You got it!");
            break 'gameloop;
        }
        guesscount += 1;
    }
}
