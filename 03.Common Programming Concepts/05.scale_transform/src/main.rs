fn main() {
    loop {
        println!("Scale transform:");
        println!("|==>1. Fahrenheit scale to degrees celsius");
        println!("|==>2. Degrees celsius to Fahrenheit scale");
        println!("|==>3. Quit");
        println!("Please input your choice:");
        
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice)
            .expect("Failed to read line");
        
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => {
                if num >= 1 && num <= 3 {
                    num
                } else {
                    print!("\x1b[2J");
                    print!("\x1b[H");
                    println!("Please input RIGHT choice!");
                    continue
                }
            },
            Err(_) => {
                print!("\x1b[2J");
                print!("\x1b[H");
                println!("Please input RIGHT choice!");
                continue
            },
        };

        if choice == 1 {
            print!("\x1b[2J");
            print!("\x1b[H");
            f_to_c();
        } else if choice == 2 {
            print!("\x1b[2J");
            print!("\x1b[H");
            c_to_f();
        } else {
            break;
        }
    }
}

fn f_to_c() {
    loop {
        println!("Please input the Fahrenheit scale:");
        
        let mut t = String::new();
        std::io::stdin().read_line(&mut t)
            .expect("Failed to read line");
        
        let t: f64 = match t.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("\x1b[2J");
                print!("\x1b[H");
                println!("Please input RIGHT temperature!");
                continue
            },
        };

        let c = (t - 32.0) * 5.0 / 9.0;

        println!("The Degrees celsius is: {}", c);

        break;
    }
}

fn c_to_f() {
    loop {
        println!("Please input the Degrees celsius:");
        
        let mut t = String::new();
        std::io::stdin().read_line(&mut t)
            .expect("Failed to read line");
        
        let t: f64 = match t.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("\x1b[2J");
                print!("\x1b[H");
                println!("Please input RIGHT temperature!");
                continue
            },
        };

        let f = t * 9.0 / 5.0 + 32.0;

        println!("The Fahrenheit scale is: {}", f);

        break;
    }
}