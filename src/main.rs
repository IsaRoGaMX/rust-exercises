fn main() {
    println!("Hello Rustaceans!");

    variables_and_constants();
    data_types();
}

fn variables_and_constants() {
    // Variables
    println!("\r\n== 01. Variables and constants ==");

    let inmutable_var = 3;
    println!("Inmutable variable value: {}", inmutable_var);

    let mut mutable_var = 4;
    println!("Mutable variable initial value: {}", mutable_var);
    mutable_var = 5;
    println!("Mutable variable final value: {}", mutable_var);

    // Constants
    const PI: f32 = 3.141592653589;
    println!("const PI value: {}\r\n", PI);

    // Shadowing
    let shadowed_variable = 6;
    println!("Shadowed variable initial value: {}", shadowed_variable);

    let shadowed_variable = 7;
    println!("Shadowed variable final value: {}", shadowed_variable);
}