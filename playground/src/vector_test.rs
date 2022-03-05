pub(crate) fn execute() {
    println!("-- VECTOR TEST --");
    basic_sample();
    with_capacity_sample1();
    with_capacity_sample2();
    with_capacity_sample3();
    with_capacity_sample4();
}

fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
    vec![0; rows * cols]
}

fn basic_sample() {
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);

    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);

    let mut buffer = new_pixel_buffer(2, 4);
    assert_eq!(buffer.len(), 2 * 4);

    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");
    assert_eq!(pal, vec!["step", "on", "no", "pets"]);

    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);

    let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
    palindrome.reverse();
    assert_eq!(palindrome, vec!["panama", "a canal", "a plan", "a man"]);
}

fn with_capacity_sample1() {
    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);

    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);

    v.push(3);
    assert_eq!(v.len(), 3);
    println!("capacity is now {}", v.capacity())
}

fn with_capacity_sample2() {
    let mut v = vec![10, 20, 30, 40, 50];
    v.insert(3, 35);
    assert_eq!(v, [10, 20, 30, 35, 40, 50]);

    v.remove(1);
    assert_eq!(v, [10, 30, 35, 40, 50]);
}

fn with_capacity_sample3() {
    let mut v = vec!["Snow Puff", "Glass Gem"];
    assert_eq!(v.pop(), Some("Glass Gem"));
    assert_eq!(v.pop(), Some("Snow Puff"));
    assert_eq!(v.pop(), None);
}

fn with_capacity_sample4() {
    let languages: Vec<String> = std::env::args().skip(1).collect();
    for l in languages {
        println!("{}: {}", l, if l.len() % 2 == 0 { "functional" } else { "imperative" })
    }
}