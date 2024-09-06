
pub mod basic {
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    pub fn check_value(x: i32) -> bool{
        if x > 50{
            return true
        }
        false
    }
}

mod advanced;
pub use self::advanced::advanced::*;

