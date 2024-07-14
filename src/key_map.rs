use crate::key_code::KeyCode;

#[derive(Clone, Debug)]
pub struct KeyMap<const W: usize, const H: usize> {
    mappings: [[KeyCode; W]; H],
}

impl<const W: usize, const H: usize> KeyMap<W, H> {
    pub const fn new(mappings: [[KeyCode; W]; H]) -> Self {
        Self { mappings }
    }

    pub fn get(&self, x: usize, y: usize) -> KeyCode {
        self.mappings[y][x]
    }
}
