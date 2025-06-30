use std::io::{self, Write};

fn main() {
    println!("🔢 Simple Calculator - Version 0.1");
    // Nhập số thứ nhất
    let num1 = read_number("Enter the first number: ");
    // Nhập số thứ hai
    let num2 = read_number("Enter the second number: ");
    // Hiển thị menu
    println!("\nSelect operation:");
    println!("1. Add (+)");
    println!("2. Subtract (-)");
    println!("3. Multiply (*)");
    println!("4. Divide (/)");
    print!("Your choice (1-4): ");
    io::stdout().flush().unwrap(); // Đảm bảo in trước khi chờ nhập
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice = choice.trim();
    match choice {
        "1" => {
            let result = num1 + num2;
            println!("✅ Result: {} + {} = {}", num1, num2, result);
        }
        "2" | "3" | "4" => {
            println!("🚧 This feature is under development. Stay tuned!");
        }
        _ => {
            println!("❌ Invalid choice. Please select 1-4.");
        }
    }
}
// Hàm đọc số từ người dùng
fn read_number(prompt: &str) -> i32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap(); // đảm bảo in ra prompt trước khi nhập

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<i32>() {
            Ok(n) => return n,
            Err(_) => {
                println!("❗ Invalid input. Please enter an integer.");
            }
        }
    }
}

