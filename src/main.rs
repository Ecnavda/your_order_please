use std::io;

fn main() {
    let mut a = String::new();

    println!("Enter a sentence.");

    io::stdin().read_line(&mut a)
        .expect("Couldn't read your input");
    
    //Removing trailing whitespace
    let a = a.trim();

    println!("{}", order(&a));
}

fn order(sentence: &str) -> String {
    let words: Vec<&str> = sentence.split(' ').collect();
    let mut output = String::new();
    //Borrowing words variable and then dereferencing the &usize from len() 
    for num in 1..(*(&words.len()) + 1) {
        for word in &words {
            if (*word.to_string()).contains(&(num.to_string())) {
                output.push_str(word);
                output.push(' ');
            }
        }
    }

    output.trim().to_string()
}
