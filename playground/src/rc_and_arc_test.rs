use std::rc::Rc;

pub fn execute() {
    println!("-- RC AND ARC TEST --");
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();

    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavor", u);

    // Rc<T> を可変参照として借用するのはできない
    // s.push_str("noodles")
}