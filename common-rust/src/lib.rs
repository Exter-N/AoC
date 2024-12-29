pub mod bit_matrix;
pub mod cc;
pub mod day;
pub mod digit;
pub mod line_stream;
pub mod mapping;
pub mod math;
pub mod mem;
pub mod multi_range;
pub mod option_flow;
pub mod ord;
pub mod point;
pub mod terrain;

pub fn unwrap_either<T>(result: Result<T, T>) -> T {
    match result {
        Ok(value) => value,
        Err(value) => value,
    }
}
