#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

extern crate rml_contracts;
use rml_contracts::*;

#[spec(ensures(result == 10))]
fn match_wildcard() -> u32 {
    let b = true;
    let x;
    match b {
        _ => x = 10,
    }
    x
}
#[spec(ensures(result == true))]
pub fn match_bool_simple() -> bool {
    let x: i32 = 5;
    let mut res_string = false;
    match x {
        1..=5 => res_string = true,
        90 => res_string = false,
        _ => res_string = false,
    }
    res_string
}
#[spec(ensures(result == true))]
fn match_bool_complex() -> bool {
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

#[spec(ensures(result == 2))]
fn match_int_simple() -> i32 {
    let x = 2;
    let a;
    match x {
        1..=5 => { a = x; }
        6 | 8 | 10 => { a = x; }
        _ => { a = -1; }
    }
    a
}

#[spec(ensures(true))]
fn match_int_complex() -> i32 {
    // multiple arms including guards
    let x = 8;
    let mut a;
    match x {
        n if n < 0 => { a = -100; }
        1..=5 => { a = x; }
        n @ 6..=10 if n % 2 == 0 => { a = n; }
        _ => { a = -1; }
    }
    a
}



 #[spec(ensures(true))]
 fn match_ident_simple() -> i32 {
     let count = 9;
     match count {
        // e @ 0 => e,
         n @ _ => n,
     }
 }

#[spec(ensures(true))]
fn match_ident_complex() -> i32 {
    let count = 9;
    let multiplier = 3;
    match count {
        e @ 0 => e * multiplier,            // returns 0
        n @ 1..=5 if { let is_big = n * multiplier > 10; is_big } => n * 2,
        m @ 6..=20 => {
            let doubled = m * 2;
            let added = doubled + multiplier;
            added
        }
        other @ _ => other,
    }
}
#[spec(ensures(true))]
fn test_all_ranges() -> i32 {
    let x:i32 = 44;
    match x {
        0..5 => 2,
        5..=10 => 3,
        11..20 => 4,
        20.. => 5,
        ..0 => 1,
    }
}


// pub enum Direction {
//     Left,
//     Right,
//     Up,
//     Down
// }
//

#[spec(
    ensures(result==true)
)]
fn simple_option(x: Option<i32>) -> bool {
    match x {
        Some(_) => true,
        None => false,
    }
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
// #[spec(ensures(true))]
// pub fn match_int() -> i32 {
//     let x = 2;
//     let a;
//     match x {
//         1..=5 => {
//             a = x;
//             //println!("Within range 1 to 5: {}", a);
//         }
//         6 | 8 | 10 => {
//             a = x;
//             //  println!("Even number smaller than 12: {}", a);
//         }
//         _ => {
//             a = -1;
//             //println!("Value Dropped!, User-defined error{}",a);
//         }
//     }
//     a
// }