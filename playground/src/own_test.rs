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
    let s2= vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
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
}
