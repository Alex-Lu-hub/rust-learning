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
        for num in (1..n + 1) {
            fibonacci_sequence.push(fibonacci(num));
        }

        let mut index = 1;
        for num in &fibonacci_sequence {
            print!("{}: {}\n", index, num);
            index = index + 1;
        }
    } 
}

// the n must less than 180!
fn fibonacci(n: u32) -> u128 {
    let mut a:u128 = 0;
    let mut b:u128 = 1;
    let mut c:u128 = 0;
    let mut count = 0;
    while count < n {
        if count == 0 {
            c = a;
        } else if count == 1 {
            c = b;
        } else {
            c = a + b;
            a = b;
            b = c;
        }
        count = count + 1;
    }
    c
}