#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

extern crate rml_contracts;
use rml_contracts::*;

#[spec(ensures(true))]
fn match_or() -> i32{
    let x:i32 = 12;
    let mut j:i32 = 0;

    match x {
        1..=10 => j = 10,
        11|12 => j = 1,
        99 => j = 12,
        100 => j = 2,
        _ => j = -1,
    };
    j
}

//println defines a unique type for the json, throws errors. Just have empty body instead.
#[spec(ensures(true))]
fn match_binding() -> i32{
    let x:i32 = 11;
    let mut y:i32 =0;

    match x {
        y @ 11 => {},
        y @ _ => {},
    }
    y
}
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
// #[spec(ensures(false))]
// fn match_bool() -> bool{
//     let x:i32=10;
//     let mut res_string = false;
//      match x {
//         1..=5 => res_string = false,
//          _ => res_string = false,
//      }
//      res_string
//  }
//
// #[spec(ensures(result == 10))]
// fn match_wildcard() -> u32{
//     let b = true;
//     let x;
//     match b{
//         _ => x = 10,
//     }
//     x
// }
// // //
// #[spec(ensures(true))]
// pub fn match_int() -> i32{
//     let x = 2;
//     let a;
//     match x{
//         1..=5 => {
//             a = x;
//             //println!("Within range 1 to 5: {}", a);
//
//         }
//         6 | 8 | 10 => {
//             a = x ;
//               //  println!("Even number smaller than 12: {}", a);
//             },
//         _ =>{
//             a = -1 ;
//             //println!("Value Dropped!, User-defined error{}",a);
//         }
//     }
//     a
// }

// REQUIRES SERDE TAG
#[spec(ensures(true))]
pub fn match_char(c: char) -> i32 {
    match c {
        'a'..='z' => 0,
        'A'..='Z' => 1,
        '0'..='9' => 2,
        '\n' => 3,
        '\t' => 4,
        ' ' => 5,
        _ => 6,
    }
}
//TODO can I test enums?
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}
//
#[spec(ensures(true))]
pub fn match_enum_simple(dir: Direction) -> u32 {
    match dir {
        Direction::Left => 0,
        Direction::Right => 1,
        Direction::Up => 2,
        Direction::Down => 3
    }
}
pub enum Status { //enum with tuple input
    Pending,
    Active{priority:u8},
    Completed{priority:u8}
}
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


