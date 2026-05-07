#[test]
fn q1() {
    let input = "Up";
    let dir: Direction;

    match input.trim() {
        "Up" => dir = Direction::Up,
        "Down" => dir = Direction::Down,
        "Left" => dir = Direction::Left,
        "Right" => dir = Direction::Right,
        _ => {
            println!("Invalid direction, defaulting to Up!");
            dir = Direction::Up;
        }
    }

    match dir {
        Direction::Up => println!("Command: Up"),
        Direction::Down => println!("Command: Down"),
        Direction::Left => println!("Command: Left"),
        Direction::Right => println!("Command: Right"),
    }

    let rec = Shape::Rectangle {
        width: 20,
        height: 30,
    };
    let circle = Shape::Circle(2.1 as f64);

    let rec_area = rec.area();
    let circle_area = circle.area();

    println!("Rectangle Area: {rec_area}");
    println!("Circle Area: {circle_area}");

    let _m1 = Message::Default(String::from("The package was delivered"));
    let _m2 = Message::System {
        customer: String::from("Dear Customer, the package was delivered"),
        dev: String::from("The package 02os2skgfpl, was delivered"),
    };

    // 16. The Option<T> is the type that'll say that a variable of type T can possibly be there or not. Result<T, E> is the result of a function that'll return either the type T or an error E

    let vec = vec![1, 2, 3, 40];
    if let Some(vl) = vec.last() {
        println!("vl inside if: {vl}");
    }

    let dbing = DBing(90, 14);
    let s_db = DB {
        server: String::from("MySQL"),
    };

    let c = Complex::DB(s_db);

    println!("dbing 0, 1: {} {}", dbing.0, dbing.1);

    match get_last() {
        Ok(l) => println!("Last is {l}"),
        Err(e) => println!("{e}"),
    }

    // 20. Enums structure can be used in a lot of dynamics way, helping not only manage structures in it, but any other type that comes besides a custom one.
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Shape {
    Circle(f64),
    Rectangle { width: i32, height: i32 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Rectangle { width, height } => (width * height) as f64,
            Shape::Circle(r) => r * r * std::f64::consts::PI,
        }
    }
}

enum Message {
    Default(String),
    System { customer: String, dev: String },
}

fn get_last() -> Result<i32, String> {
    let v = vec![1, 2, 3, 4, 999];
    let l = v.last();

    match l {
        Some(l) => Ok(*l),
        None => Err(String::from("Can't find last")),
    }
}

struct DB {
    server: String,
}

struct DBing(i32, i32);

enum Complex {
    DB(DB),
}
