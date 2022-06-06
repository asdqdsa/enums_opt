fn main() {

    // let five = Some(5);
    // let six = plus_one(five);
    let none = plus_one(&None);
    let result = plus_one(&none);
    dbg!(result);

    // let my_penny = Coin::Penny;
    let my_quarter = Coin::Quarter(State::Russia);
    let coin_up = get_coin(&my_quarter);
    dbg!(coin_up);
    let sum = None;
    let show = adding_opt(&sum, &coin_up);
    dbg!(show);


}



fn adding_opt(y: &Option<i32>, test: &i32) -> Option<i32> {
    match y {
        Some(z) => Some(z + test),
        None => None,
    }
}



#[derive(Debug)]
enum State {
    Russia,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn plus_one(x: &Option<i32>) -> Option<i32> {
    let temp : i32 = 4;
    match x {
        Some(var) => Some(var + temp),
        None => None,
    }
}

fn get_coin(coin: &Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:#?}", state);
            25
        },

    }
}

// fn collect(x: Option<i32>, state: ) {

// }
