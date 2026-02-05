use std::future::pending;
use std::ptr::null;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn match_bool(cond: bool) -> &'static str { //unnecessary casting, works if fn returns bool as well
    let res_string;
    match cond {
        true => res_string = "true",
        false => res_string = "false",
    }
    res_string
}

pub fn match_int(point: i32) -> i32{
    let x = point;
    let a;
    match x{
        1..=5 => {
            a = x;
            println!("Within range 1 to 5: {}", a);

                }
        _ =>{
            a = -1 ;
            println!("Value Dropped! Result is: {}",a);
            }
        }
    a
}

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

pub fn match_char(c: char) -> &'static str {
    match c {
        'a'..='z' => "lowercase",
        'A'..='Z' => "uppercase",
        '0'..='9' => "digit",
        '\n' => "newline",
        '\t' => "tab",
        ' ' => "space",
        _ => "other char",
    }
}
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

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


pub enum Status { //enum with tuple input
    Pending,
    Active{priority:u8},
    Completed(String)
}
pub fn match_enum_with_vals(item: (Status, i32)){
    match item{
        (Status::Pending, count) if count > 100 =>
            {println!("Large Backlog!:{}",count);},
        (Status::Active {priority:  1 ..=3},_) =>
            {println!("High Prio Task");},
        (Status::Completed(ref msg),count) => {
            println!("Done: '{}', processed {} items", msg,count); },
        (Status,count) => {
            println!("Special case, count: {}",count);
        }
    }
}

fn match_deep_nested_tuple(data: (
    ((((((&str, i32), bool), f64), char), Vec<i32>), Option<String>),
    Result<u32, String>
)) -> String {
    match data {
        // The ONE positive path - 7 layers deep
        (
            (
                (
                    (
                        (
                            (
                                ("magic", 42),
                                true
                            ),
                            3.14
                        ),
                        'X'
                    ),
                    vec
                ),
                Some(s)
            ),
            Ok(num)
        ) if vec.len() == 3 && s == "secret" && num > 100 => {
            "SUCCESS: All 7 layers matched!".to_string()
        }
        // Everything else fails
        _ => "FAIL".to_string()
    }
}

/// Test cases start from below ///
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn basic_match_pass(){
        let result = match_int(2);
        assert_eq!(result, 2);
    }
    
    #[test]
    fn basic_match_fail(){
        let result = match_int(100);
        assert_ne!(result, 100);
    }

    #[test]
    fn match_int_lower_bound() {
        assert_eq!(match_int(1), 1);
    }

    #[test]
    fn match_int_upper_bound() {
        assert_eq!(match_int(5), 5);
    }

    #[test]
    fn match_int_out_of_range() {
        assert_eq!(match_int(10), -1);
    }


    #[test]
    fn test_match_float_zero() {
        assert_eq!(match_float(0.0), 0.0);
    }

    #[test]
    fn test_match_float_fraction() {
        assert_eq!(match_float(0.5), 0.5);
    }

    #[test]
    fn test_match_float_negative() {
        assert_eq!(match_float(-5.0), f32::MIN);
    }

    #[test]
    fn test_match_char_lowercase() {
        assert_eq!(match_char('m'), "lowercase");
    }

    #[test]
    fn test_match_char_digit() {
        assert_eq!(match_char('7'), "digit");
    }

    #[test]
    fn test_match_char_special() {
        assert_eq!(match_char('!'), "other char");
    }

    
    #[test]
    fn test_pending_large_backlog() {
        // Positive: Pending with count > 100
        match_enum_with_vals((Status::Pending, 150));
        // Should print "Large Backlog!:150"
    }

    #[test]
    fn test_high_priority_active() {
        // Positive: Active with priority 1-3
        match_enum_with_vals((Status::Active { priority: 2 }, 50));
        // Should print "High Prio Task"
    }

    #[test]
    fn test_pending_small_count() {
        // Negative: Pending but count <= 100, hits wildcard
        match_enum_with_vals((Status::Pending, 50));
        // Should print "Special case, count: 50"
    }

    #[test]
    fn test_completed_with_message() {
        // Positive: Completed variant
        match_enum_with_vals((Status::Completed("Success".to_string()), 42));
        // Should print "Done: 'Success', processed 42 items"
    }

    #[test]
    fn test_active_low_priority() {
        // Negative: Active but priority not in 1-3 range, hits wildcard
        match_enum_with_vals((Status::Active { priority: 5 }, 10));
        // Should print "Special case, count: 10"
    }


    #[test]
    fn test_success_path() {
        let data = (
            (
                (
                    (
                        (
                            (
                                ("magic", 42),
                                true
                            ),
                            3.14
                        ),
                        'X'
                    ),
                    vec![1, 2, 3]
                ),
                Some("secret".to_string())
            ),
            Ok(101)
        );
        assert_eq!(match_deep_nested_tuple(data), "SUCCESS: All 7 layers matched!");
    }

    #[test]
    fn test_fail_path() {
        let data = (
            (
                (
                    (
                        (
                            (
                                ("wrong", 42),
                                true
                            ),
                            3.14
                        ),
                        'X'
                    ),
                    vec![1, 2, 3]
                ),
                Some("secret".to_string())
            ),
            Ok(101)
        );
        assert_eq!(match_deep_nested_tuple(data), "FAIL");
    }
}
