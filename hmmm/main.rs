use std::io;

fn main() {
    let discovered_string = "______".to_string();
    println!("Hello, you're in a hangman game!");
    println!("The word so far is {}", discovered_string);

    let secret_string = "rustiscute".to_string();
    let mut guessed_letters = "".to_string();
    let mut guessed_checked= vec![];
    let mut guesses = 5;

    loop {

        let mut new_string = "rustiscute".to_string();
        let mut guessed_wrong = true;
        // println!("The word so far is {}", discovered_string);
        println!("You have {} guesses left", guesses);
        println!("You have guessed the following letters: {}", guessed_letters);
        println!("Please guess a letter:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: char = match guess.trim().parse() {
            Ok(char) => char,
            Err(_) => continue
        };

        // guessed_letters.append(&mut guess);
        if secret_string.contains(guess){
            guessed_checked.append(&mut vec![guess]);
            guessed_wrong = false;
        }
        guessed_letters += &guess.to_string();

        for i in secret_string.chars() {
            if guessed_checked.contains(&i) {
                continue
            }
            else {
                new_string = new_string.replace(i, "_");

            }
        }

        if guessed_wrong {
            guesses -= 1;
        }

        if new_string == secret_string{
            println!("You won! You discovered the word {}. Congrats!", secret_string);
            break
        }
        if guesses == 0{
            println!("You lost:(");
            break
        }
        println!("You guessed {}", new_string);
    }
    }
