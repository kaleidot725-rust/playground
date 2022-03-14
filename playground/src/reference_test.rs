use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

pub fn execute() {
    println!("-- REFERENCE TEST --");
    table();
    table2();
    and_mut();
    substitution();
    reference();
    comparison();
    share_reference();
}

fn table() {
    println!("-- Table --");
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(), vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(), vec!["The Musicians".to_string(), "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(), vec!["Perseus with the head of Medusa".to_string(), "a salt cellar".to_string()]);
    show(table); // この時点で table は未初期化になるので、今後は使えない
}

fn table2() {
    println!("-- Table2 --");
    let mut table2 = Table::new();
    table2.insert("Gesualdo".to_string(), vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
    table2.insert("Caravaggio".to_string(), vec!["The Musicians".to_string(), "The Calling of St. Matthew".to_string()]);
    table2.insert("Cellini".to_string(), vec!["Perseus with the head of Medusa".to_string(), "a salt cellar".to_string()]);
    sort_works(&mut table2);
    show2(&table2);
}


fn show(table: Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn show2(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_aritis, works) in table {
        works.sort();
    }
}

fn and_mut() {
    println!("-- & and &mut test --");
    let x = 10;
    let r = &x;

    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert_eq!(*m, 64);

    struct Anime { name: &'static str, bechdel_pass: bool }
    let aria = Anime { name: "Aria: The Animation", bechdel_pass: true };
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");
    assert_eq!((*anime_ref).name, "Aria: The Animation");
}

fn substitution() {
    println!("-- substitution --");
    let b = true;
    let x = 10;
    let y = 20;
    let mut r = &x;
    if b { r = &y; }
    assert!(*r == 10 || *r == 20);
}

fn reference() {
    println!("-- reference --");

    struct Point { x: i32, y: i32}
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;
    assert_eq!(rrr.y, 729);
}

fn comparison() {
    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert_eq!(rrx, rry);
    assert_eq!(rx, ry);
    assert!(!std::ptr::eq(rx, ry));
    // assert!(rx == rrx); 型違うのでエラーになる {integer} == &{integer}
    assert_eq!(rx, *rrx);
}

fn share_reference() {
    fn factorial(n: usize) -> usize {
        (1..n+1).product()
    }

    // このサンプルと等価
    // let r = &factorial(6);
    // let p = 1009;
    // assert_eq!(r + &p, 1729);

    let r = &factorial(6);
    assert_eq!(r + &1009, 1729);
}