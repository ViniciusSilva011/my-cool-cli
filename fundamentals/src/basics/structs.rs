struct Book {
    title: String,
    author: String,
    page: u32,
}

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn area2(&mut self) {}
}

struct Color(i32, i32, i32);

#[derive(Clone)]
struct User {
    name: String,
    age: i32,
}

fn longest<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() { x } else { y }
}

struct Usera<'a> {
    name: &'a str,
}

#[test]
fn t1() {
    let book = Book {
        title: String::from("Prince Of Percia"),
        author: String::from("Vinicius"),
        page: 40,
    };

    println!("book.title: {}", book.title);
    println!("book.author: {}", book.author);
    println!("book.page: {}", book.page);

    let rec = Rectangle::new(10, 10);

    println!("Area: {}", rec.area());

    // A tuple doesn't have explicit properties
    let _color = Color(1, 2, 3);

    let user1 = User {
        name: String::from("Vinicius"),
        age: 26,
    };

    let user2 = User {
        age: 20,
        ..user1.clone()
    };

    println!("user1.name: {}", user1.name);
    println!("user1.age: {}", user1.age);
    println!("user2.name: {}", user2.name);
    println!("user2.age: {}", user2.age);

    let v1 = String::from("Small");
    let v2 = String::from("BiiiiG");
    let res: &String;

    {
        res = longest(&v1, &v2);
    }

    println!("res: {res}");

    let name = "Vanacius";
    let usera = Usera { name: name };
    println!("usera.name: {}", usera.name);

    // 9. I think that's because rust makes it strict to avoid unsafe memory changes
}
