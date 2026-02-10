#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

extern crate rml_contracts;
use rml_contracts::*;

#[spec(ensures(true))]
fn foo() -> Option<u32> {
    let x = Some(1u32) ;
    let y: Option<bool> = None;
    x
}
#[spec(ensures(true))]
fn bar() -> i32 {
    let x: i32 = 23;
    x
}

// //
#[spec(ensures(true))]
fn match_bool(cond: bool) -> bool{
    let res_string;
     match cond {
         true => res_string = true,
         false => res_string = false,
     }
     res_string
 }
// //
#[spec(ensures(true))]
pub fn match_int() -> i32{
    let x = 2;
    let a;
    match x{
        1..=5 => {
            a = x;
            //println!("Within range 1 to 5: {}", a);

        }
        6 | 8 | 10 =>
            {a =x;
              //  println!("Even number smaller than 12: {}", a);
            },
        _ =>{
            a = -1 ;
            //println!("Value Dropped!, User-defined error{}",a);
        }
    }
    a
}
#[spec(ensures(true))]
pub fn match_float(point: f32) -> f32{
    let res:f32;
    match point {
        f if f == 0.0 => res = f,
        f if f > 0.0 && f < 1.0 => {res = f; print!("fraction")},
        f if f.is_nan() =>{res = f; print!("not a number")},
        _ => {res = f32::MIN; print!("other float, saving as smallest showable number")}
    }
    res
}
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
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

#[spec(ensures(true))]
pub fn match_enum_simple(dir: Direction) -> &'static str {
    let res_string;
    match dir {
        Direction::Left => {res_string = "left"},
        Direction::Right => {res_string = "right"},
        Direction::Up => {res_string = "up"},
        Direction::Down => {res_string = "down"}
        _ => {res_string = "other direction"}
    }
    res_string
}
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
