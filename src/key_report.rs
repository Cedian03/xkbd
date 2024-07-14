use usbd_hid::descriptor::KeyboardReport;

use crate::key_code::KeyCode;
use crate::key_map::KeyMap;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KeyReport {
    modifiers: u8,
    key_codes: [u8; 6]
}

impl KeyReport {
    pub fn build<const W: usize, const H: usize>(key_scan: &[[bool; W]; H], key_map: &KeyMap<W, H>) -> Self {
        let mut modifiers = 0;
        let mut key_codes = [0; 6];
        let mut key_count = 0;
    
        'outer: for y in 0..H {
            for x in 0..W {
                if !key_scan[y][x] {
                    continue;
                }
    
                let key_code = key_map.get(x, y);
    
                if key_code.is_none() {
                    continue;
                }

                if let Some(mask) = key_code.modifier_mask() {
                    modifiers |= mask;
                    continue;
                }
    
                if key_count >= key_codes.len() {
                    key_codes.fill(u8::from(KeyCode::RollOverError));
                    break 'outer;
                }
    
                key_codes[key_count] = u8::from(key_code);
                key_count += 1;
            }
        }
    
        Self { modifiers, key_codes }
    }

    pub fn is_empty(&self) -> bool {
        self.key_codes.iter().all(|x| *x == u8::from(KeyCode::None))
    }
}

impl From<KeyReport> for KeyboardReport {
    fn from(r: KeyReport) -> Self {
        KeyboardReport {
            modifier: r.modifiers,
            reserved: 0,
            leds: 0,
            keycodes: r.key_codes,
        }
    }
}
