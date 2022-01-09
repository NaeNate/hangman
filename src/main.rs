use std::io;

fn main() {
    println!("Welcome to hangman. Please enter a word.");

    let mut fails = 0;
    let mut guesses = Vec::new();
    let mut word = String::new();

    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");

    let mut answer = vec![String::from("_"); word.trim().len()];

    print!("{}[2J", 27 as char);

    loop {
        println!("Please enter a letter.");

        let mut letter = String::new();

        io::stdin()
            .read_line(&mut letter)
            .expect("Failed to read line");

        let letter = letter[..1].to_lowercase().chars().next().unwrap();

        if !(guesses.contains(&letter.to_string())) {
            guesses.push(letter.to_string());
        }

        let mut correct = false;

        for (i, char) in word.chars().enumerate() {
            if letter == char {
                answer[i] = char.to_string();
                correct = true;
            }
        }

        if !(correct) {
            fails += 1;
        }

        println!(
            "
                +---+
                |   |
                {}   |
               {}{}{}  |
               {} {}  |
                    |
               =======
               {}
               {}
               ",
            {
                if fails > 0 {
                    "o"
                } else {
                    " "
                }
            },
            {
                if fails > 2 {
                    "/"
                } else {
                    " "
                }
            },
            {
                if fails > 1 {
                    "|"
                } else {
                    " "
                }
            },
            {
                if fails > 3 {
                    "\\"
                } else {
                    " "
                }
            },
            {
                if fails > 4 {
                    "/"
                } else {
                    " "
                }
            },
            {
                if fails > 5 {
                    "\\"
                } else {
                    " "
                }
            },
            answer.join(""),
            guesses.join(",")
        );

        if fails == 6 {
            println!("You loose. :(");
            break;
        } else if !(answer.contains(&String::from("_"))) {
            println!("You win! :)");
            break;
        }
    }
}
