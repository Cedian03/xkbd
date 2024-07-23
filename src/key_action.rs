use crate::key_code::KeyCode;

#[derive(Copy, Clone, Debug)]
pub enum KeyAction {
    Key(Option<KeyCode>),
}