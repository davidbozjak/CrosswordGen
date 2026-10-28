use crate::crossword::ClueDirection;

pub fn get_incrementors_for_direction(direction: ClueDirection) -> (fn(i32)->i32, fn(i32)->i32) {
    match direction {
        ClueDirection::Horizontal => (|w| w + 1, |w| w),
        ClueDirection::Vertical => (|w| w, |w| w + 1)
    }
}