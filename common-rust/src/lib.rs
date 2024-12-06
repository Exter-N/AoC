pub mod bit_matrix;
pub mod cc;
pub mod day;
pub mod line_stream;
pub mod mapping;
pub mod math;
pub mod multi_range;
pub mod ord;
pub mod point;
pub mod terrain;

pub fn unwrap_either<T>(result: Result<T, T>) -> T {
    match result {
        Ok(value) => value,
        Err(value) => value,
    }
}
