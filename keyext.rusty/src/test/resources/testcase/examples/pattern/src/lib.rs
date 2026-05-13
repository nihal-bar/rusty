#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

extern crate rml_contracts;
use rml_contracts::*;

// #[spec(ensures(true))]
// fn foo() -> Option<u32> {
//     let x = Some(1u32) ;
//     let y: Option<bool> = None;
//     x
// }
// #[spec(ensures(true))]
// fn bar() -> i32 {
//     let x: i32 = 23;
//     let y:i32 = 1002;
//     let mut z:i32;
//     let this_is_boolean = true;
//
//     if this_is_boolean {
//         z = y/x;
//     } else {
//         z = x + 1;
//     }
//     z
// }

// //
#[spec(ensures(false))]
pub fn match_bool_simple() -> bool {
    let x: i32 = 10;
    let mut res_string = true;
    match x {
        1..=5 => res_string = true,
        _ => res_string = false,
    }
    res_string
}


#[spec(ensures(result == 10))]
fn match_wildcard() -> u32 {
    let b = true;
    let x;
    match b {
        _ => x = 10,
    }
    x
}

#[spec(ensures(true))]
pub fn match_int() -> i32 {
    let x = 2;
    let a;
    match x {
        1..=5 => {
            a = x;
            //println!("Within range 1 to 5: {}", a);
        }
        6 | 8 | 10 => {
            a = x;
            //  println!("Even number smaller than 12: {}", a);
        }
        _ => {
            a = -1;
            //println!("Value Dropped!, User-defined error{}",a);
        }
    }
    a
}
#[spec(ensures(true))]
fn match_bool_complex() -> bool {
    // more branches and nested matching
    let x: i32 = 3;
    let mut res = false;
    match x {
        0 => res = false,
        1..=3 => match x {
            2 => res = true,
            _ => res = true,
        },
        _ => res = false,
    }
    res
}

#[spec(ensures(true))]
fn match_bool_complex_2() -> bool {
    let x: i32 = 3;
    let mut res = false;
    match x {
        0 => res = false,
        1..=3 => res = true,
        4..=10 => res = false,
        _ => res = false,
    }
    res
}

// #[spec(ensures(true))]
// pub fn match_float(point: f32) -> f32{
//     let res:f32;
//     match point {
//         f if f == 0.0 => res = f,
//         f if f > 0.0 && f < 1.0 => {res = f; print!("fraction")},
//         f if f.is_nan() =>{res = f; print!("not a number")},
//         _ => {res = f32::MIN; print!("other float, saving as smallest showable number")}
//     }
//     res
// }
//
// #[spec(ensures(true))]
// pub fn match_char(c: char) -> &'static str {
//     match c {
//         'a'..='z' => "lowercase",
//         'A'..='Z' => "uppercase",
//         '0'..='9' => "digit",
//         '\n' => "newline",
//         '\t' => "tab",
//         ' ' => "space",
//         _ => "other char",
//     }
// }
// pub enum Direction {
//     Left,
//     Right,
//     Up,
//     Down
// }
//
// #[spec(ensures(true))]
// pub fn match_enum_simple(dir: Direction) -> &'static str {
//     let res_string;
//     match dir {
//         Direction::Left => {res_string = "left"},
//         Direction::Right => {res_string = "right"},
//         Direction::Up => {res_string = "up"},
//         Direction::Down => {res_string = "down"}
//         _ => {res_string = "other direction"}
//     }
//     res_string
// }
//
//
// pub enum Status { //enum with tuple input
//     Pending,
//     Active{priority:u8},
//     Completed(String)
// }
//
// #[spec(ensures(true))]
// pub fn match_enum_with_vals(item: (Status, i32)){
//     match item{
//         (Status::Pending, count) if count > 100 =>
//             {println!("Large Backlog!:{}",count);},
//         (Status::Active {priority:  1 ..=3},_) =>
//             {println!("High Prio Task");},
//         (Status::Completed(ref msg),count) => {
//             println!("Done: '{}', processed {} items", msg,count); },
//         (Status,count) => {
//             println!("Special case, count: {}",count);
//         }
//     }
// }
//
// #[spec(ensures(true))]
// fn match_deep_nested_tuple(data: (
//     ((((((&str, i32), bool), f64), char), Vec<i32>), Option<String>),
//     Result<u32, String>
// )) -> String {
//     match data {
//         // The ONE positive path - 7 layers deep
//         (
//             (
//                 (
//                     (
//                         (
//                             (
//                                 ("magic", 42),
//                                 true
//                             ),
//                             3.14
//                         ),
//                         'X'
//                     ),
//                     vec
//                 ),
//                 Some(s)
//             ),
//             Ok(num)
//         ) if vec.len() == 3 && s == "secret" && num > 100 => {
//             "SUCCESS: All 7 layers matched!".to_string()
//         }
//         // Everything else fails
//         _ => "FAIL".to_string()
//     }
// }
//

#[spec {
    ensures(result)
    }]
fn simple_option(x: Option<i32>) -> bool {
    match x {
        Some(_) => true,
        None => true
    }
}