fn main() {
    loop{
        print!("\x1b[2J");
        print!("\x1b[H");
        println!("Using for get Pig Latin!");
        println!("You can input:");
        println!("|==>A list of word(Use space to split).");
        println!("|==>Quit. Quit the program.");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "Quit" {
            break;
        }

        let mut pig_latin_string = String::new();

        for word in input.trim().split_whitespace() {
            pig_latin_string.push_str(&PigLatin(word));
            pig_latin_string.push(' ');
        }

        println!("The pig latin string is: {}", pig_latin_string);
        println!("Press ENTER to Continue!");

        std::io::stdin().read_line(&mut input)
            .expect("Failed to read line");
    }
}

fn PigLatin(s: &str) -> String {
    let v = vec!["a", "e", "i", "o", "u", "A", "E", "I", "O", "U"];
    let first = &s[0..1];
    let rest = &s[1..];

    if (first >= "a" && first <= "z") || (first >= "A" && first <= "Z") {
        if v.contains(&first) {
            return format!("{}-{}", s, "hay");
        } else {
            return format!("{}-{}{}", rest, first, "ay");
        }
    }

    return s.to_string();
}
