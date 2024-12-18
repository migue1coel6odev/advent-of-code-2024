#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Facing {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

pub enum CacheItem {
    FINAL(usize),
    WALL,
    POS((usize, usize), usize),
}

pub enum CacheItemV2 {
    FINAL(usize),
    WALL,
    POS(Facing, (usize, usize), usize),
}

pub enum Found {
    FoundWithX(Vec<usize>),
    FoundWithNoX(Vec<usize>),
}
