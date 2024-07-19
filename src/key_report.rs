use crate::key_code::KeyCode;
use crate::key_map::KeyMap;

pub const REPORT_SIZE: usize = 8;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KeyReport {
    modifiers: u8,
    codes: [KeyCode; 6]
}

impl KeyReport {
    pub fn build<const W: usize, const H: usize>(scan: &[[bool; W]; H], map: &KeyMap<W, H>) -> Self {
        let mut modifiers = 0;
        let mut codes = [KeyCode::None; 6];
        let mut count = 0;
    
        'outer: for y in 0..H {
            for x in 0..W {
                if !scan[y][x] {
                    continue;
                }
    
                let code = map.get(x, y);
    
                if code.is_none() {
                    continue;
                }

                if let Some(mask) = code.modifier_mask() {
                    modifiers |= mask;
                    continue;
                }
    
                if count >= codes.len() {
                    codes.fill(KeyCode::RollOverError);
                    break 'outer;
                }
    
                codes[count] = code;
                count += 1;
            }
        }
    
        Self { modifiers, codes }
    }

    pub fn pack(&self) -> [u8; REPORT_SIZE] {
        [
            self.modifiers,
            0,
            self.codes[0].into(),
            self.codes[1].into(),
            self.codes[2].into(),
            self.codes[3].into(),
            self.codes[4].into(),
            self.codes[5].into(),
        ]
    }

    pub fn is_empty(&self) -> bool {
        self.modifiers == 0 && self.codes.iter().all(|x| *x == KeyCode::None) 
    }
}
