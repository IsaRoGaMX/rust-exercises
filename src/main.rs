use std::u8;

fn main() {
    println!("Hello Rustaceans!");

    variables_and_constants();
    scalar_data_types();
    compound_types();
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

fn scalar_data_types() {
    println!("\r\n== 02. Variables and constants ==");

    let i8_var: i8 = i8::MAX;
    let u8_var: u8 = u8::MAX;
    println!("i8 max value: {}, u8 max value: {}", i8_var, u8_var);

    
    let i16_var: i16 = i16::MAX;
    let u16_var: u16 = u16::MAX;
    println!("i16 max value: {}, u16 max value: {}", i16_var, u16_var);

    let i32_var: i32 = i32::MAX;
    let u32_var: u32 = u32::MAX;
    println!("i32 max value: {}, u32 max value: {}", i32_var, u32_var);

    let i64_var: i64 = i64::MAX;
    let u64_var: u64 = u64::MAX;
    println!("i64 max value: {}, u64 max value: {}", i64_var, u64_var);

    let i128_var: i128 = i128::MAX;
    let u128_var: u128 = u128::MAX;
    println!("i128 max value: {}, u128 max value: {}\r\n", i128_var, u128_var);

    let f32_var: f32 = f32::MAX;
    let f64_var: f64 = f64::MAX;
    println!("f32 max value: {}, f64 max value: {}\r\n", f32_var, f64_var);

    let implicit_bool_variable = false;
    let explicit_bool_variable: bool = true;
    println!("Implicit variable value: {}, Explicitd variable value: {}\r\n", implicit_bool_variable, explicit_bool_variable);
    
    let c = 'z';
    let z = 'ℤ';
    let emoji = '😻';
    println!("ASCII char: {}, Unicode char: {}, Emoji: {}", c, z, emoji);
}

fn compound_types() {
    println!("\r\n== 03. Compond Types ==");

    // implicit tuple
    let tuple = (200.34, 87, -344.9);
    let (x, y, z) = tuple; // destructiring
    println!("Player 1 Coords: ({}, {}, {})", x, y, z);

    // explicit tuple
    let tuple_two: (f32, f32, f32) = (-128.12, 98.0, 123.01);
    println!("Player 2 Coords: ({}, {}, {})", tuple_two.0, tuple_two.1, tuple_two.2);

    // implicit array
    let implicit_array = [1, 2, 3, 4];
    println!("Implicit array: {}", implicit_array.map(|n| n.to_string()).join(","));

    // explicit array
    let explicit_array: [i32; 4] = [1, 2, 3, 4];
    println!("Explicit array: {}", explicit_array.map(|n| n.to_string()).join(","));

    let array_zero = [0; 5];
    println!("Initialized array: {}", array_zero.map(|n| n.to_string()).join(","));
}

