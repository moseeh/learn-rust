use std::io;

fn main() {
    let mut i = 0;
    let answer = "The letter e";
    let mut guess = String::new();
    loop {
        println!(
            "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?"
        );
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read the line");
        if guess == answer {
            println!("Number of trials: {}", i);
            break;
        } else {
            guess.clear();
            i = i + 1;
        }
    }
}
