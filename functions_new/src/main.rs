fn main() {

    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);

    println!("the test passed dude!");
}

fn celsius_to_fahrenheit(temp: f64) -> f64 {
    temp *1.8 + 32.0
}