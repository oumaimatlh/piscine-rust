use std::io;

fn main() {
    let mut response = String::new();
    let mut count = 0;
    loop {
        count += 1;

        println!(
            "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?"
        );
        io::stdin().read_line(&mut response).expect("Error");
        if response.trim() == "e" {
            println!("Number of trials: {}", count);
            break;
        }
        response = String::new()
    }
}
