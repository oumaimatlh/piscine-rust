use question_mark::*;

fn main() {
   let d = Flag::opt_flag("diff", "gives the difference between two numbers");
    println!("short hand: {}, long hand: {}, description: {}", d.short_hand, d.long_hand, d.desc);
    // output: "short hand: -d, long hand: --diff, description: gives the difference between two numbers"

}
