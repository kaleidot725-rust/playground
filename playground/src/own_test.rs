pub(crate) fn execute() {
    println!("-- OWN TEST --");
    print_padovan();
    assert_box();
    print_composers();
    own_move_test();
}

fn print_padovan() {
    let mut padovan = vec![1, 1, 1];
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
}

fn assert_box() {
    let point = Box::new((0.625, 0.5));
    let label = format!("{:?}", point);
    assert_eq!(label, "(0.625, 0.5)");
}

fn print_composers() {
    struct Person {
        name: String,
        birth: i32,
    }

    let mut composers = Vec::new();
    composers.push(Person { name: "Palestrina".to_string(), birth: 1525 });
    composers.push(Person { name: "Dowland".to_string(), birth: 1563 });
    composers.push(Person { name: "Lully".to_string(), birth: 1632 });

    for composer in composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
}

fn own_move_test() {
    // t に代入した際に s が move する、そのため u に代入する際には s が未初期化状態になるのでエラーとなる
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s;
    // let u = s;

    // もし t と u の両方に s を渡したいのであれば clone する必要がある
    let s2 = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t2 = s2.clone();
    let u2 = s2.clone();

    // 新しい String を代入したとき
    let mut s3 = "Govinda".to_string();
    s3 = "Siddhartha".to_string(); // この時点で古い値がドロップする

    let mut s4 = "Govinda".to_string();
    let t4 = s4; // t4 に move する
    s4 = "Siddharth".to_string(); // s4 に新しい値を代入する
    let u4 = s4; // u4 に move する、s4 は未初期化状態ではないのでエラーにはならない。
}

fn own_move_and_control_flow_test() {
    // let x  = vec![10, 20, 30];
    // if c {
    //     f(x); ここで x を移動している
    // } else {
    //     g(x); ここで x を移動している
    // }
    //
    // h(x); 必ず x は移動することになるので、ここで未初期化状態になる

    // let x = vec![10, 20, 30];
    // while f() {
    //     g(x); 1週目で x が移動してしまうので2週目では未初期化状態になる
    // }

    // let mut x = vec![10, 20, 30];
    // while f() {
    //     g(x); ここで x を移動する
    //     x = h(); ここで x に新しい値を入れるので 2 週目移行でも未初期化状にならない
    // }
    // e(x);

    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }
    let third = &v[2]; // 参照をつけなければインデックス参照できない
    let fifth = &v[4]; // 参照をつけなければインデックス参照できない

    let mut v2 = Vec::new();
    for i in 101..106 {
        v2.push(i.to_string());
    }

    let fifth = v2.pop().expect("vector empty!");
    assert_eq!(fifth, "105");

    let second = v2.swap_remove(1);
    assert_eq!(second, "102");

    let third = std::mem::replace(&mut v2[2], "substitue".to_string());
    assert_eq!(third, "103");

    assert_eq!(v2, vec!["101", "104", "substitute"]);

    let v3 = vec!["liberte".to_string(), "egalite".to_string(), "fraternite".to_string()];
    for mut s in v3 {
        s.push('!');
        println!("{}", s);
    }

    struct Person {
        name: Option<String>,
        birth: i32,
    }
    let mut composers = Vec::new();
    composers.push(Person { name: Some("Palestrina".to_string()), birth: 1525 });
    let first_name = std::mem::replace(&mut composers[0].name, None);
    assert_eq!(first_name, Some("Palestrina".to_string()));
    assert_eq!(composers[0].name, None);
}
