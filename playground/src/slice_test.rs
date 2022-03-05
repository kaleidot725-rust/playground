pub(crate) fn execute() {
    println!("-- SLICE TEST --");
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];

    println!("&a");
    print(&a);

    println!("&v");
    print(&v);

    println!("&v[0..2]");
    print(&v[0..2]);

    println!("&a[2..]");
    print(&a[2..]);

    println!("&v[1..3]");
    print(&v[1..3]);
}

fn print(n: &[f64]) {
    for elt in n {
        println!("{}", elt);
    }
}