use std::collections::HashMap;
use std::fmt::format;

use crate::enum_test::Ordering::{Equal, Greater, Less};
use crate::enum_test::Shape::{Cuboid, Sphere};

// 指定しない場合はコンパイラが0から埋めてくれる
enum Ordering {
    Less,
    Equal,
    Greater,
}

fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Less
    } else if n > m {
        Greater
    } else {
        Equal
    }
}

// このような形で自分で値を決められる
enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

// ==演算子などのサポートは明示的に記述する必要がある
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

// データを保持する列挙型
#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

// 構造体を保持する列挙型も作れる
struct Point3d {
    x: f32,
    y: f32,
    z: f32,
}

enum Shape {
    Sphere { center: Point3d, radius: f32 },
    Cuboid { corner1: Point3d, corner2: Point3d },
}

// Json を定義するための構造体も楽に書ける
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

// ジェネリック列挙型も使える
enum Option<T> {
    None,
    Some(T),
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum BinaryTree<T> {
    Empty,
    NotEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

// fn rough_time_to_english(rt: RoughTime) -> String {
//     match rt {
//         RoughTime::InThePast(units, count) => format!("{} {} ago", count, units.plural()),
//         RoughTime::JustNow => format!("just now"),
//         RoughTime::InTheFuture(unit, 1) => format!("a {} from now", unit.singular()),
//         RoughTime::InTheFuture(units, count) => format!("{} {} from now", count, units.plural()),
//     }
// }

fn rabbit_count_to_english() {
    let rabbit_count = 10;
    match rabbit_count {
        0 => {}
        1 => println!("A rabbit is nosing around in the clover."),
        n => println!("There are {} rabbits hopping about in the meadow", n),
    }
}

fn describe_point(x: i32, y: i32) -> &'static str {
    use std::cmp::Ordering::*;
    match (x.cmp(&0), y.cmp(&0)) {
        (Equal, Equal) => "at the origin",
        (_, Equal) => "on the x axis",
        (Equal, _) => "on the y axis",
        (Greater, Greater) => "in the first quadrant",
        (Less, Greater) => "in the second quadrant",
        _ => "somewhere else"
    }
}

fn greet_people(names: &[&str]) {
    match names {
        [] => { println!("Hello, nobody") }
        [a] => { println!("Hello, {}.", a) }
        [a, b] => { println!("Hello, {} and {}.", a, b) }
        [a, .., b] => { println!("Hello, everyone from {} to {}.", a, b) }
    }
}

pub fn execute() {
    let four_score_and_seven_years_ago = RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);
    let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);
    let sphere = Sphere { center: Point3d { x: 0f32, y: 0f32, z: 0f32 }, radius: 4f32 };
    let cuboid = Cuboid { corner1: Point3d { x: 0f32, y: 0f32, z: 0f32 }, corner2: Point3d { x: 0f32, y: 0f32, z: 0f32 } };
}





