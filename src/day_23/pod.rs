
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PodKind {
    Amber = 1,
    Bronze = 10,
    Copper = 100,
    Desert = 1000
}
impl PodKind {
    pub fn val(&self) -> u32 {
        *self as u32
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pod {
    kind: PodKind,
    total_moves: u32,
    pub coords: (u32, u32),
}
impl Pod {
    pub fn new(kind: PodKind, coords: (u32, u32)) -> Pod {
        Pod {
            kind,
            total_moves: 0,
            coords
        }
    }
    pub fn do_move(&mut self) {
        self.total_moves += self.kind.val()
    }
    pub fn get_total_move_costs(&self) -> u32 {
        self.total_moves
    }
}