use crate::key_action::KeyAction;

#[derive(Clone, Debug)]
pub struct KeyMap< const W: usize, const H: usize> {
    mappings: [[KeyAction; W]; H],
}

impl<const W: usize, const H: usize> KeyMap<W, H> {
    pub const fn new(mappings: [[KeyAction; W]; H]) -> Self {
        Self { mappings }
    }

    pub fn get(&self, x: usize, y: usize) -> KeyAction {
        self.mappings[y][x]
    }
}
