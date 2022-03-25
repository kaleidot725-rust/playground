pub fn execute() {
    println!("-- STRUCT TEST --");

    let width = 1024;
    let height = 576;
    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };

    assert_eq!(image.size, (1024, 576));
    assert_eq!(image.pixels.len(), 1024 * 576);

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

// 名前フィールド付構造体
struct GrayscaleMap {
    pixels: Vec<u8>,
    // 構造体はデフォルトでプライベートになる
    size: (usize, usize), // 構造体はデフォルトでプライベートになる
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