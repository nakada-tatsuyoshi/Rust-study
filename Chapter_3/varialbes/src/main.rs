fn main() {
    let x = 1+3*2;
    println!("{x}");
    let t = true;
    let f: bool = false;
    println!("{t} {f}");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{0}", tup.1);
    another_function(tup.0);
    let y = {
        let x = 3;
        x + 1 //式の終端にはセミコロンがつかない。セミコロンがつくと文になり、値を返さなくなる。
    };
    println!("{y}");

    println!("フィボナッチ数列の10番目は{}です。", fibonacci(10));

    if_esle_roop();
    println!("摂氏100度は華氏{}度です。", celcius_to_fahrenheit(100.0));
}

fn another_function(x: i32) { //引数は必ず型宣言する必要あり。
    println!("おはよう。{x}");
}

//文と式。文：何らかの動作をして値を返さない命令。式：結果値になるのもの。式は値を返す。文は値を返さない。
fn if_esle_roop() {
    let number = 3;
    if number < 5 { //if文の条件式はbool型でなければならない。理論値以外の値が自動的に理論値に変換されることはない。複数の条件文があったら、最初に条件がtrueに評価されたものだけ実行され、あとはチェックしない。
        println!("condtion was true");
    } else {
        println!("condtion was false");
    }
    for number in (1..4).rev() {
        println!("{number}!");
    }
}

fn celcius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

fn fibonacci(n: u32) -> u32 {
    if n <=1 {
        n
    } else {
        fibonacci(n-1) + fibonacci(n-2)
    }
}
