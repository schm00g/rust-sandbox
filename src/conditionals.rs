// Conditionals - Used to check condition of something and act on the result

pub fn run(){
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person_is_of_age = true;

    // If/else
    if age >= 21 && check_id || knows_person_is_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: I cannot serve you alcohol.");
    } else {
        println!("Bartender: I'll need to see your ID");
    }

    // Shorthanf if
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
}
