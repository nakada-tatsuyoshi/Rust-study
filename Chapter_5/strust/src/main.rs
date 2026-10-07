fn main() {
    let username = String::from("someusername123");
    let email = String::from("someemail@example.com");
    let user1 = build_user(email, username);
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1  //構造体の更新構文。user1の必要なフィールドをmoveしている。、emailだけ変更する。
    };
    //println!("build_user関数で構造体を作成しました{0}", user1.username); //user1のusernameはmoveされているので、ここでは使えない。


    //タプル構造体
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:#?}", rect1);

}

//構造体
//タプルと同様、構造体の一部を異なる型にできる。異なる点は、各データ片に名前を付けられる。
//タプルは不変、配列は可変。構造体は？

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    let user1 = User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    };
    user1
}

//タプル構造体
