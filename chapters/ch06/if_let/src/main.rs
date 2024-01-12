#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // using else
    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

// ***** notes *****

/*
    - `if` `let` syntax lets you combine into a less verbose way
    -  `if` `let` is  a `syntax sugar` for a `match` that runs code when the value matches one pattern and then ignores all other values.
*/