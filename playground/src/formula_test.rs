pub fn execute() {
    println!("-- FORMULA TEST --");
    formula_test();
    if_test();
    shadowing_test();
    match_test();
    if_let_test();
    loop_test();
    closure_test();
}

fn formula_test() {
    println!("-- formula --");
    // セミコロンありの場合は値が返らない
    // セミコロンなしの場合は値が返り msg に代入される
    let msg = {
        let test = "TEST1";
        "TEST2"
    };
    println!("{}", msg);
}

fn if_test() {
    println!("-- if --");
    // 変数を宣言しておいて、一度だけ初期化する
    // 一度だけ初期化するので mut などの記載もいらない
    let success = true;
    let name;
    if success {
        name = "SUCCESS"
    } else {
        name = "FAILED"
        // execute FAILED function
    }
}

fn shadowing_test() {
    println!("shadowing");
    // for の line を新たな変数宣言で置き換える
    // これはシャドーイングと呼ばれておりよく Rust では用いられる書き方
    let lines = ["a", "b", "c", "d"];
    for line in lines {
        let line = line.to_uppercase();
        println!("{}", line);
    }
}

fn match_test() {
    println!("match");
    // match 式で複数パターンの分岐を表現できる、アンダースコアはワイルドカードを表す
    let code = 0;
    match code {
        0 => println!("OK"),
        1 => println!("Wires STangled"),
        2 => println!("User Asleep"),
        _ => println!("Unrecognized Error {}", code)
    }
}

fn if_let_test() {
    println!("if let");

    let some = Some("Please help me.");
    if let Some(message) = some {
        println!("MESSAGE {}", message);
    } else {
        println!("NONE MESSAGE");
    };
}

fn loop_test() {
    println!("loop");

    // ..演算子は範囲(range)を生成する
    for i in 0..20 {
        println!("{}", i);
    }

    let strings: Vec<String> = vec!["TEST1".to_string(), "TEST2".to_string(), "TEST3".to_string()];

    // コレクションの参照に対してループしないと、ドロップする
    // for s in strings {
    //     println!("{}", s);
    // }
    // println!("{} errors(s)", strings.len());

    // なのでドロップしないように参照を参照する
    for s in &strings {
        println!("{}", s);
    }
    println!("{} errors(s)", strings.len());

    // loop で break に値を与えると loop 式の値になる
    // let answer  = loop {
    //     if let Some(line) = next_line() {
    //         if line.start_with("answer:") {
    //             break line;
    //         } else {
    //             break "answer: nothing";
    //         }
    //     }
    // }

    // return 四季は現在の関数から脱出し、値を呼び出し元に戻す
    return;
}

fn closure_test() {
    println!("closure");
    let is_even = |x: u64| -> bool { x % 2 == 0 };
    assert_eq!(is_even(14), true);
}