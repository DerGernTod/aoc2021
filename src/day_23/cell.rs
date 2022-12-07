use std::fmt;

use super::pod::PodKind;
#[derive(PartialEq, Eq, Debug)]
pub enum CellType {
    Hallway,
    Entry,
    Goal(PodKind),
    Wall
}
#[derive(PartialEq, Eq, Debug)]
pub struct Cell {
    pub occupant: Option<usize>,
    pub cell_type: CellType,
    pub location: (u32, u32)
}
impl Cell {
    pub fn new(occupant: Option<usize>, cell_type: CellType, location: (u32, u32)) -> Cell {
        Cell {
            occupant,
            cell_type,
            location
        }
    }
}
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let print_char = if let Some(pod) = &self.occupant { pod.to_string() }
            else if matches!(self.cell_type, CellType::Hallway | CellType::Entry | CellType::Goal(_)) { String::from(".") }
            else { String::from("#") };
        write!(f, "{:}", print_char)
    }
}