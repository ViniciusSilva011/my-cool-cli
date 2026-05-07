#[test]
fn q1() {
    let mut x = 5;
    println!("x1: {x}");
    x = 6;

    println!("x2: {x}");
}

#[test]
fn q2() {
    let mut name = String::from("Ana");
    name.push_str(" Silva");

    println!("name: {name}");
}

#[test]
// 3.
fn q3() {
    let mut _x = 10;
    println!("x: {_x}"); // 20
    let y = &_x;

    // x = 20;

    println!("y: {y}"); // 20
}

#[test]
fn q4() {
    let mut s = String::from("hi");
    let _r1 = &s;
    let _r2 = &mut s;
    // println!("r1: {r1}"); // fail
}

#[test]
fn q5() {
    let mut n = 3;
    let r = &mut n;
    *r += 1;
    println!("n: {n}");
}

#[test]
fn q6() {
    let mut items = vec![1, 2, 3];
    items.push(4);
    let _li = items.last();

    if let Some(_li) = items.last() {
        println!("4: {_li}");
    } else {
        println!("No items");
    }

    match items.last() {
        Some(_li) => println!("match: {_li}"),
        None => println!("No items"),
    }
}

#[test]
fn q7() {
    let v: Vec<i32> = Vec::new();

    println!("{v:?}");
}

#[test]
fn q8() {
    let _x: i32 = 5;
    let mut _x = _x;
}

#[test]
fn q9() {
    let mut s = String::from("Yes");
    q9h(&mut s);

    println!("s: {s}")
}
fn q9h(s: &mut String) {
    s.push('!');
}

#[test]
fn q10() {
    let mut s = String::from("Yes");
    q10h(&mut s);

    println!("s: {s}")
}
fn q10h(s: &mut String) {
    s.push('!');
}

#[test]
fn q11() {
    let mut n = 10;
    q11h(&mut n);

    println!("n: {n}");
}

fn q11h(n: &mut i32) {
    *n += 1;
}

#[test]
fn q12() {
    let mut s = String::from("Mickey");

    {
        let b = &mut s;

        b.push('!');
        // borrow ends
    }
    println!("b: {s}")
}

#[test]
fn q13() {
    let mut a = 1;
    let b = &mut a;
    // let c = &mut a;

    println!("b: {b}")
}

#[test]
fn q14() {
    let mut a = String::from("Kokoro");
    let b = &mut a;
    // let c = &mut a;

    println!("b: {b}")
}

#[test]
fn q15() {
    let mut s = String::from("Gorloc");
    {
        let r = &mut s;
        r.push_str(" The destroyer!");
    }
}

// 1. x isn't set as "mut"
// 2. to change it should have "mut" in it
// 3. can't change a value of a borrowed variable. but if that was possible, it would print 20 I think;
// 4. I think you can't borrow a borrowed variable with a mutable change and then print the old variable that borrowed it.
// 5. Yes, you're changing the value inside the reference r is pointing to.
// 6. mut
// 7. No, because to reference it as mutable, the variable also need the "mut".
// 8... In a normal programming language that shouldn't  be possible, declaring the same variable twice. explain to me the difference. I know the "mut" makes the variable mutable, because it is a constant every time
// 9. Fix: s: &mut String. add &mut
// 10. It is missing "mut" as in: &mut s
// 11.
//    code:
//       fn q11h(n: &mut i32) {
//           *n += 1;
//       }
// 12.
//    code:
//       fn q12() {
//           let mut s = String::from("Mickey");
//           let b = &mut s;
//           b.push('!');
//           println!("b: {b}")
//       }
// 13. No because you're not printing the last borrower.
// 14. If you want to store the mutable reference you need to explicitly set it.
// 15. The string is being pushed through the reference, not the "r" variable that is being changed. The small scope also includes the bigger scope it is in too.
