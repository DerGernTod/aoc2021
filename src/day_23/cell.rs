use crate::day_23::pod::Pod;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub occupant: Option<Pod>,
    pub top: bool,
    pub bot: bool,
    pub left: bool,
    pub right: bool
}
impl Cell {
    pub fn new(occupant: Option<Pod>, top: bool, bot: bool, left: bool, right: bool) -> Cell {
        Cell {
            occupant,
            top,
            bot,
            left,
            right
        }
    }
}