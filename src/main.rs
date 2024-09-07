use std::collections::HashMap;

fn main (){
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("The Blue Team score is: {score}");
    
    println!("---------------------------------");
    
    for (key, value) in &scores {
        println!("The {key} Team score is: {value}");
    }

}