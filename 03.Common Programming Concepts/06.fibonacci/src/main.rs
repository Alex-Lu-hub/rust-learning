fn main() {
    loop{
        println!("Using for Get Fibonacci Sequence!");
        println!("You can input:");
        println!("|==>Number(1~180). The length you want.");
        println!("|==>Quit. Quit the program.");
    
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        
        if input.trim() == "Quit" {
            break;
        }

        // the n must less than 180!
        let n: u32 = match input.trim().parse() {
            Ok(num) => {
                if num > 0 && num < 181 {
                    num
                } else {
                    print!("\x1b[2J");
                    print!("\x1b[H");
                    println!("Please input RIGHT number!");
                    continue
                }
            },
            Err(_) => {
                print!("\x1b[2J");
                print!("\x1b[H");
                println!("Please input RIGHT command!");
                continue
            },
        };
        
        let mut fibonacci_sequence: Vec<u128> = Vec::new();

        let mut a:u128 = 0;
        let mut b:u128 = 1;
        let mut count = 0;
        while count < n {
            if count == 0 {
                fibonacci_sequence.push(a);
            } else if count == 1 {
                fibonacci_sequence.push(b);
            } else {
                fibonacci_sequence.push(a + b);
                let tmp = a + b;
                a = b;
                b = tmp;
            }
            count = count + 1;
        }

        let mut index = 1;
        for num in &fibonacci_sequence {
            println!("{}: {}", index, num);
            index = index + 1;
        }
    } 
}
