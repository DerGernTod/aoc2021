#[derive(PartialEq, Eq, Debug)]
pub struct NumberInfo {
    pub id: usize,
    pub value: usize,
    pub depth: usize,
    pub parent: Option<usize>
}

#[derive(PartialEq, Eq, Debug)]
pub struct PairInfo {
    pub id: usize,
    pub left: usize,
    pub right: usize,
    pub depth: usize,
    pub parent: Option<usize>
}

impl NumberInfo {
    pub fn new(id: usize, value: usize, depth: usize, parent: Option<usize>) -> NumberInfo {
        NumberInfo {
            id,
            value,
            depth,
            parent
        }
    }
}

impl NumberEntry {
    pub fn set_offset(&mut self, offset: usize) {
        match self {
            NumberEntry::Literal(l) => {
                l.id += offset;
                if let Some(parent_id) = l.parent {
                    l.parent = Some(parent_id + offset);
                }
            },
            NumberEntry::Pair(p) => {
                p.id += offset;
                p.left += offset;
                p.right += offset;
                if let Some(parent_id) = p.parent {
                    p.parent = Some(parent_id + offset);
                }
            },
            NumberEntry::None => ()
        };
    }
    pub fn get_id(&self) -> usize {
        match self {
            NumberEntry::Literal(l) => l.id,
            NumberEntry::Pair(p) => p.id,
            NumberEntry::None => 0
        }
    }

    pub fn left(&self) -> usize {
        if let NumberEntry::Pair(p) = self {
            return p.left;
        }
        panic!("Number entry {:?} is not a pair!", self);
    }

    pub fn right(&self) -> usize {
        if let NumberEntry::Pair(p) = self {
            return p.right;
        }
        panic!("Number entry {:?} is not a pair!", self);
    }

    pub fn value(&self) -> usize {
        if let NumberEntry::Literal(l) = self {
            return l.value;
        }
        panic!("Error getting value of {:?} which not a literal! Id: {}", self, self.get_id());
    }

    pub fn get_depth(&self) -> usize {
        match self {
            NumberEntry::Literal(l) => l.depth,
            NumberEntry::Pair(p) => p.depth,
            NumberEntry::None => panic!("None has no depth!")
        }
    }

    pub fn get_parent(&self) -> Option<usize> {
        match &self {
            NumberEntry::Literal(l) => l.parent,
            NumberEntry::None => None,
            NumberEntry::Pair(p) => p.parent
        }
    }

    pub fn set_parent(&mut self, parent: Option<usize>) {
        match self {
            NumberEntry::Literal(l) => l.parent = parent,
            NumberEntry::Pair(p) => p.parent = parent,
            _ => ()
        }
    }

    pub fn literal(&self) -> Option<&NumberInfo> {
        if let NumberEntry::Literal(l) = self {
            Some(l)
        } else {
            None
        }
    }
}

impl PairInfo {
    pub fn new(id: usize, left: usize, right: usize, depth: usize, parent: Option<usize>) -> PairInfo {
        PairInfo {
            id,
            left,
            right,
            depth,
            parent
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum NumberEntry {
    Literal(NumberInfo),
    Pair(PairInfo),
    None
}