mod murmur;
mod database;

fn main() {
    println!("Welcome to the LinkedList program.");
    println!("As seen in Forbes, this is the best program in the world.");

    let mut db = database::Database::new();

    loop {
        println!("1. Add");
        println!("2. Get");
        println!("3. Print");
        println!("0. Exit");
        print!("Select an option: ");

        let mut user_choice_input = String::new();
        std::io::stdin().read_line(&mut user_choice_input).expect("Failed to read input");

        let user_choice = match user_choice_input.trim().parse::<i64>() {
            Ok(value) => {
                match value {
                    0..=3 => value,
                    _ => {
                        println!("Invalid input. Please provide a valid option.");
                        continue
                    },
                }
            },
            Err(_) => {
                println!("Invalid input. Please provide a valid option.");
                continue
            },
        };

        match user_choice {
            0 => {
                println!("Bye!");
                break
            },
            1 => {
                println!("Enter a key to add to the database:");
                let key = match read_user_integer() {
                    Some(key) => key,
                    None => {
                        println!("Invalid input. Please enter a valid number.");
                        continue
                    }
                };

                println!("Enter the value:");
                match read_user_integer() {
                    Some(value) => db.add(key, value),
                    None => {
                        println!("Invalid input. Please enter a valid number.");
                        continue
                    },
                }
            },
            2 => {
                println!("Enter a key to search the database:");
                let key = match read_user_integer() {
                    Some(key) => key,
                    None => {
                        println!("Invalid input. Please enter a valid number.");
                        continue
                    },
                };
                
                match db.get(key) {
                    Some(value) => println!("Value: {}", value),
                    None => println!("Value not found."),
                }
            },
            3 => {
                println!("Current data:");
                db.print_all();
            }
            _ => panic!()
        }
    }
}

fn read_user_integer() -> Option<i64> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");

    match input.trim().parse::<i64>() {
        Ok(value) => Some(value),
        Err(_) => None
    }
}
