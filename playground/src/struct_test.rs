pub fn execute() {
    println!("-- STRUCT TEST --");
    grayscale_sample();
    broom_sample();
    impl_sample();
}

// 名前フィールド付構造体
struct GrayscaleMap {
    pixels: Vec<u8>,
    // 構造体はデフォルトでプライベートになる
    size: (usize, usize), // 構造体はデフォルトでプライベートになる
}

fn grayscale_sample() {
    println!("-- grayscale_sample --");

    let width = 1024;
    let height = 576;
    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };

    assert_eq!(image.size, (1024, 576));
    assert_eq!(image.pixels.len(), 1024 * 576);
}

struct MonoMap {
    pub pixels: Vec<bool>,
    // pub をつけることで外部モジュールから見えるようになる
    pub size: (usize, usize), // pub をつけることで外部モジュールから見えるようになる
}

struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

#[derive(Copy, Clone)]
enum BroomIntent { FetchWater, DumpWater }

fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom { height: b.height / 2, ..b };
    let mut broom2 = Broom { name: broom1.name.clone(), ..broom1 };

    broom1.name.push_str(" I");
    broom2.name.push_str(" II");

    (broom1, broom2)
}

fn broom_sample() {
    println!("-- broom_sample --");

    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater
    };

    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.height, 30);
    assert_eq!(hokey1.health, 100);
    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey2.height, 30);
    assert_eq!(hokey2.health, 100);
}

// タプル型構造体
struct Bounds(usize, usize); // Bounds の中身も pub にできる

fn bounds_sample() {
    let image_bounds = Bounds(1024, 768);
    assert_eq!(image_bounds.0 * image_bounds.1, 786432);
}

// ユニット型構造体は何も宣言しない構造体
struct Onesuch;

fn onesuch_sample() {
    let o = Onesuch;
}

fn impl_sample() {
    println!("-- impl_sample --");

    let mut q = Queue::new();

    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));

    q.push('∞');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('∞'));
    assert_eq!(q.pop(), None);

    assert!(q.is_empty());
    q.push('T');
    assert!(!q.is_empty());
    q.pop();

    q.push('P');
    q.push('D');
    assert_eq!(q.pop(), Some('P'));
    q.push('X');

    let (older, younger) = q.split();
    assert_eq!(older, vec!['D']);
    assert_eq!(younger, vec!['X']);
}
pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { older: Vec::new(), younger: Vec::new()}
    }

    pub fn push(&mut self, t: T) {
        self.younger.push(t);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }

}

pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    const UNIT: Vector2 = Vector2 { x: 1.0, y: 0.0 };
}
