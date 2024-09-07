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
    
    println!("---------------------------------");
    
    let field_name = String::from("Favorite color");
    let field_value = String::from("Red");
    
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    
    //we can not use field_name and field_value anymore, because they moved into the map
    // println!("{field_name}");
    // println!("{field_value}");
    println!("---------------------------------");

    //Overwriting values
    scores.insert(String::from("Blue"), 20);
    scores.insert(String::from("Blue"), 35);

    println!("{scores:?}");
    
    println!("---------------------------------");

    //Adding a key and value only if key isn't present
    scores.entry(String::from("Blue")).or_insert(105);
    scores.entry(String::from("Purple")).or_insert(105);

    println!("{scores:?}");
    
    println!("---------------------------------");

    //Updating a value based on the old value
    let text = "hello world wonderful world";

    let mut hash_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = hash_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{hash_map:?}");
    
}