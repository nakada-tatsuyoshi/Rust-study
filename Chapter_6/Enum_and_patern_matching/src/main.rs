fn main() {
    match_expression();
    if_let_expression();
}
//列挙型

//列挙型は、複数の型の値を一つの型としてまとめることができる。
//IPアドレス
//

fn IP_adress_enum() {
    enum IpAddrKind {
        V4(String), //enumの各列挙子に直接データを格納できる。構造体でもほかのenumでも、なんでも含めめる。
        V6(String),
    }
    let four = IpAddrKind::V4(String::from("127.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
}

//Optiion型、値があるかないかを表す型。Some(T)とNoneの二つの値を持つ。Rustにはnullがない。

fn option_enum() {
    enum Option<T> { //ジェネリック型。Tは任意の型を表す。
        None,
        Some(T),
    }
}

//match式
fn match_expression() {
    enum Coin {
        Penny, 
        Nickel, 
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin { //match式は、あらゆる可能性を網羅しつくす必要がある。
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => {
                println!("Dimeです");
                10
            },
            Coin::Quarter => 25,
        }
    }
    let coin = Coin::Dime;
    let value = value_in_cents(coin);
    println!("value is {0}", value);
}

//if let記法でifとletをより冗長性の少ない方法で組み合わせる。
fn if_let_expression() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}