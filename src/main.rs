use talking::*;

fn main() {
    println!("{:?}", talking("Hello how are you?"));
// Sure.

println!("{:?}", talking("Are you okay?"));
// Sure.

println!("{:?}", talking("WHAT ARE YOU DOING?"));
// Quiet, I am thinking!

println!("{:?}", talking("IS IT WORKING?"));
// Quiet, I am thinking!

println!("{:?}", talking("Is EVERYTHING okay?"));
// Sure.

println!("{:?}", talking("123?"));
// Sure.

println!("{:?}", talking("HELLO 123?"));
// Quiet, I am thinking!

println!("{:?}", talking("hello?"));
// Sure.

println!("{:?}", talking("Hello? "));
// Sure. (à tester avec trim)

println!("{:?}", talking("   ?"));
// Sure. (à vérifier selon la logique de l'exercice)
}
