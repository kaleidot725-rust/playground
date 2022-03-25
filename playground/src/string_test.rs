pub fn execute() {
    println!("-- STRING TEST --");

    // 文字列リテラル
    let speech = "\"Ouch!]: said the well.\n";
    println!("In the room the women come and go, Singing of Mount Abora");
    println!("It was a bright, cold day in April, and \
        there were four of us-\
        more or less");

    // 生文字列
    let default_win_install_path = r"C:\Program Files\Gorillas";
    println!(r###"
            This raw string started with 'r###'.
            Therefore it does not end until we reach a quote mark ('"')
            followed immediately by three pound signs ('###'):
        "###);

    // バイト文字列 (b をつける、または rb で生バイト文字列にもなる)
    let get_method = b"GET";
    assert_eq!(get_method, &[b'G', b'E', b'T']);

    // ASCCI や Emoji かで文字列のバイト数が動的に変わる
    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    let people = "🙆";

    println!("noodles.len {}", noodles.len()); // 複数文字で ASCII なので長さが 7
    println!("oodles.len {}", oodles.len()); // 複数文字で ASCII なので長さが 6
    println!("people.len {}", people.len()); // 一文字だけど Emoji なので長さが 4

    // String について
    let error_message = "too many pets".to_string();
    assert_eq!(format!("{}° {:02}’ {:02}” N", 24, 5, 23), "24° 05’ 23” N");

    let bits = vec!["veni", "vidi", "vici"];
    assert_eq!(bits.concat(), "venividivici");
    assert_eq!(bits.join(", "), "veni, vidi, vici");

    // 文字列の == と != 演算子のサポート
    assert_eq!("ONE".to_lowercase(), "one");
    assert_eq!("     clean\n".trim(), "clean");

    for word in "veni, vidi, vici".split(", ") {
        assert!(word.starts_with("v"));
    }
}