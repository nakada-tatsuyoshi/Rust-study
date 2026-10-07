fn main() {
    let s = String::from("hello");

    takes_ownership(s); // sはムーブされるので、ここではもう使えない。

    // println!("String型はmoveされるので、使えなくなる。{s}"); // ここでsを使うとコンパイルエラーになる。

    let x = 5;

    makes_copy(x); // xはCopyされるので、ここでも使える。
    println!("i32型はCopyされるので、使える。{x}");

    //ここから参照について
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    slice_example();

}

fn takes_ownership(s: String) {
    println!("String型はmoveされるので、使えなくなる。{s}");
}

fn makes_copy(x: i32) {
    println!("i32型はCopyされるので、使える。{x}");
}

fn calculate_length(s: &String) -> usize { //&記号が参照を表す。参照を生成することを借用と呼ぶ。参照は不変。&mut Stringで可変参照となる。可変参照は2つ作れない。
    s.len()
}

fn slice_example() {
    let s = String::from("hello world");

    println!("String: {s}");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("Hello: {hello}, World: {world}");
}