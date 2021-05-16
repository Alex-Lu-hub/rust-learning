use std::collections::HashMap;

fn main() {
    loop{
        print!("\x1b[2J");
        print!("\x1b[H");
        println!("Using for number process to get avg, mid, mode!");
        println!("You can input:");
        println!("|==>A list of number(Use space to split).");
        println!("|==>Quit. Quit the program.");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let mut number_vec = Vec::new();
        let mut flag = 0;

        if input.trim() == "Quit" {
            break;
        }

        for num in input.trim().split_whitespace() {
            let n: i32 = match num.parse() {
                Ok(number) => {
                    flag = 1;
                    number
                },
                Err(_) => {
                    print!("\x1b[2J");
                    print!("\x1b[H");
                    println!("Please input RIGHT command!");
                    flag = -1;
                    break
                },
            };
            number_vec.push(n);
        }

        if flag != 1 {
            continue;
        }

        let avg = GetAvgNum(&number_vec);
        let mid = GetMidNum(&mut number_vec);
        let mode = GetModeNum(&number_vec);
    
        println!("The avg: {}", avg);
        println!("The mid: {}", mid);
        println!("The mode: {}", mode);
        println!("Press ENTER to Continue!");

        std::io::stdin().read_line(&mut input)
            .expect("Failed to read line");
    }
}    

fn GetAvgNum(v: &Vec<i32>) -> f64 {
    let mut count = 0;
    let mut sum = 0;
    for i in v {
        sum = sum + i; 
        count = count + 1;
    }
    let ret: f64 = sum as f64 / count as f64;
    ret
}

fn GetMidNum(v: &mut Vec<i32>) -> f64 {
    let len = v.len();
    for i in 0..len {
        for j in i..len {
            if v[j] < v[i] {
                let tmp = v[j];
                v[j] = v[i];
                v[i] = tmp;
            }
        }
    }

    let len_mod = len % 2; 

    let ret: f64 = if let 0 = len_mod {
        (v[len / 2] as f64 + v[len / 2 - 1] as f64) / 2.0
    } else {
        v[len / 2] as f64
    };
    
    ret
}

fn GetModeNum(v: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for num in v {
        let count = map.entry(num).or_insert(1);
        *count += 1;
    }

    let mut ret = 0;
    let mut count = 0;

    for (key, value) in &map {
        if *value > count {
            count = *value;
            ret = **key;
        }
    }

    ret
}
