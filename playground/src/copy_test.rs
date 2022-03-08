pub fn execute() {
    println!("-- COPY TEST --");

    // この処理の書き方だと Label は Copy 型ではないので println でエラーになる
    // struct Label {
    //     number: u32,
    // }
    //
    // fn print(l: Label) { println!("STAMP: {}", l.number) }
    //
    // let l = Label { number: 3 };
    // print(l);
    // println!("My label number is : {}", l.number);

    #[derive(Copy, Clone)]
    struct Label {
        number: u32,
    }

    fn print(l: Label) { println!("STAMP: {}", l.number) }

    let l = Label { number: 3 };
    print(l);
    println!("My label number is : {}", l.number);

    // 構造体の中身が Copy 型ではない場合には derive(Copy, Clone) をつけても上手く行かない
    // #[derive(Copy, Clone)]
    // struct StringLabel { name: String};
}