use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

static mut STASH: &i32 = &128;
static mut WORTH_POINTING_AT: i32 = 1000;

pub fn execute() {
    println!("-- REFERENCE TEST --");
    table();
    table2();
    and_mut();
    substitution();
    reference();
    comparison();
    share_reference();
    unsafe {
        static_scope(&WORTH_POINTING_AT);
    }
    share_and_change();
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
    println!("-- comparison --");

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
    println!("-- share_reference --");

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

fn local_scope() {
    println!("-- local_scope --");

    // このコードだど X の依存期間が終了後にアクセスしてしまうのでコンパイルエラーになる
    // {
    //     let r;
    //     {
    //         let x = 1;
    //         r = &x;
    //     }
    //     assert_eq!(*r, 1);
    // }

    // このコードであればXの生存期間が終了していないのでエラーにならない
    {
        let x =1;
        {
            let r = &x;
            assert_eq!(*r, 1);
        }
    }
}

fn static_scope(p: &'static i32) {
    println!("-- static_scope --");

    unsafe {
        STASH = p; // ここで渡される p の生存期間も static である必要がある
    }
}

fn share_and_change() {
    println!("-- share_and_change --");

    // r の参照している v が未初期化になるのでエラーになる
    // let v = vec![4, 8 , 19, 27, 34, 10];
    // let r = &v;
    // let aside = v;
    // r[0];

    // このように記述すると r の生存期間が終わってから aside に代入するので問題がでなくなる
    let v = vec![4, 8, 19, 27, 34, 10];
    {
        let r = &v;
        r[0];
    }
    let aside = v;
}

fn extend_test() {
    println!("-- extend_test --");
    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];

    extend(&mut wave, &head);
    extend(&mut wave, &tail);

    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);

    // Vecを変更しながら読み出す処理はエラーになる
    // なぜなら可変参照と共有参照を同時に取得できないから
    // extend(&mut wave, &wave);
    // assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0]);
}

fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt)
    }
}


