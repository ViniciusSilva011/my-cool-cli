#[test]
fn move_string() {
    let name = String::from("yes");
    let _n2 = name;

    println!("{}", _n2);
    // can't print name
}

//#region "Ownership, borrowing, and moves"
#[test]
fn borrow() {
    let name = String::from("Vinicius");
    print_name(&name);
}

fn print_name(name: &String) {
    println!("name: {name}")
}

#[test]
fn modify_ownership() {
    let mut name = String::from("my owner");

    mownership(&mut name);

    println!("{name}");
}
fn mownership(name: &mut String) {
    name.push_str(" Vinicius");
}

#[test]
fn return_modified_ownership() {
    let name = String::from("FOOOR ");
    let changed = rmo(name);

    println!("{changed}");
}

fn rmo(name: String) -> String {
    let sparta = name.clone() + "SPARTAAAAA!";

    sparta
}

#[test]
fn get_size() {
    let howbig = String::from("PAASSSSWWWWOORRRDDAS£@£§");
    let len = gs(&howbig);

    println!("length: {len}");
}

fn gs(value: &String) -> usize {
    value.len()
}

#[test]
fn borrowing_rules() {
    let mut txt = String::from("THIS IS TXT!");
    let a = &txt;
    let b = &txt;

    println!("a: {a}, b: {b}");

    let c = &mut txt;

    c.push_str("YES!");
    // println!("a: {a}, b: {b}"); CAN'T RUN
}

#[test]
fn borrowing_rules2() {
    let mut v = vec![1, 2, 3];
    let mut v2 = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);

    let f = &v[0];

    println!("{f}");

    v.push(4);
    // println!("{f}"); CAN'T RUN
}
//#endregion

//#region "Mutability"

//#endregion
