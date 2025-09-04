use enigo::Key as EnigoKey;
use crate::enums::{Key, KeyLocation};

/// 键盘映射工具（将逻辑按键映射到平台特定的键码）
pub struct KeyMapper;

impl KeyMapper {
    pub fn new() -> Self {
        KeyMapper
    }

    /// 将逻辑按键映射到Enigo键码
    pub fn map_key(&self, key: Key, location: Option<KeyLocation>) -> Option<EnigoKey> {
        match key {
            Key::Backspace => Some(EnigoKey::Backspace),
            Key::Tab => Some(EnigoKey::Tab),
            Key::Enter => Some(EnigoKey::Return),
            Key::Shift => match location {
                Some(KeyLocation::Left) => Some(EnigoKey::LShift),
                Some(KeyLocation::Right) => Some(EnigoKey::RShift),
                _ => Some(EnigoKey::Shift),
            },
            Key::Ctrl => match location {
                Some(KeyLocation::Left) => Some(EnigoKey::LControl),
                Some(KeyLocation::Right) => Some(EnigoKey::RControl),
                _ => Some(EnigoKey::Control),
            },
            Key::Alt => match location {
                Some(KeyLocation::Left) => Some(EnigoKey::Alt),
                Some(KeyLocation::Right) => Some(EnigoKey::Alt),
                _ => Some(EnigoKey::Alt),
            },
            Key::CapsLock => Some(EnigoKey::CapsLock),
            Key::Escape => Some(EnigoKey::Escape),
            Key::Space => Some(EnigoKey::Space),
            Key::PageUp => Some(EnigoKey::PageUp),
            Key::PageDown => Some(EnigoKey::PageDown),
            Key::End => Some(EnigoKey::End),
            Key::Home => Some(EnigoKey::Home),
            Key::ArrowLeft => Some(EnigoKey::LeftArrow),
            Key::ArrowUp => Some(EnigoKey::UpArrow),
            Key::ArrowRight => Some(EnigoKey::RightArrow),
            Key::ArrowDown => Some(EnigoKey::DownArrow),
            Key::PrintScreen => Some(EnigoKey::Print),
            Key::Insert => Some(EnigoKey::Insert),
            Key::Delete => Some(EnigoKey::Delete),
            Key::Num0 => Some(EnigoKey::Unicode('0')),
            Key::Num1 => Some(EnigoKey::Unicode('1')),
            Key::Num2 => Some(EnigoKey::Unicode('2')),
            Key::Num3 => Some(EnigoKey::Unicode('3')),
            Key::Num4 => Some(EnigoKey::Unicode('4')),
            Key::Num5 => Some(EnigoKey::Unicode('5')),
            Key::Num6 => Some(EnigoKey::Unicode('6')),
            Key::Num7 => Some(EnigoKey::Unicode('7')),
            Key::Num8 => Some(EnigoKey::Unicode('8')),
            Key::Num9 => Some(EnigoKey::Unicode('9')),
            Key::A => Some(EnigoKey::Unicode('a')),
            Key::B => Some(EnigoKey::Unicode('b')),
            Key::C => Some(EnigoKey::Unicode('c')),
            Key::D => Some(EnigoKey::Unicode('d')),
            Key::E => Some(EnigoKey::Unicode('e')),
            Key::F => Some(EnigoKey::Unicode('f')),
            Key::G => Some(EnigoKey::Unicode('g')),
            Key::H => Some(EnigoKey::Unicode('h')),
            Key::I => Some(EnigoKey::Unicode('i')),
            Key::J => Some(EnigoKey::Unicode('j')),
            Key::K => Some(EnigoKey::Unicode('k')),
            Key::L => Some(EnigoKey::Unicode('l')),
            Key::M => Some(EnigoKey::Unicode('m')),
            Key::N => Some(EnigoKey::Unicode('n')),
            Key::O => Some(EnigoKey::Unicode('o')),
            Key::P => Some(EnigoKey::Unicode('p')),
            Key::Q => Some(EnigoKey::Unicode('q')),
            Key::R => Some(EnigoKey::Unicode('r')),
            Key::S => Some(EnigoKey::Unicode('s')),
            Key::T => Some(EnigoKey::Unicode('t')),
            Key::U => Some(EnigoKey::Unicode('u')),
            Key::V => Some(EnigoKey::Unicode('v')),
            Key::W => Some(EnigoKey::Unicode('w')),
            Key::X => Some(EnigoKey::Unicode('x')),
            Key::Y => Some(EnigoKey::Unicode('y')),
            Key::Z => Some(EnigoKey::Unicode('z')),
            Key::F1 => Some(EnigoKey::F1),
            Key::F2 => Some(EnigoKey::F2),
            Key::F3 => Some(EnigoKey::F3),
            Key::F4 => Some(EnigoKey::F4),
            Key::F5 => Some(EnigoKey::F5),
            Key::F6 => Some(EnigoKey::F6),
            Key::F7 => Some(EnigoKey::F7),
            Key::F8 => Some(EnigoKey::F8),
            Key::F9 => Some(EnigoKey::F9),
            Key::F10 => Some(EnigoKey::F10),
            Key::F11 => Some(EnigoKey::F11),
            Key::F12 => Some(EnigoKey::F12),
            Key::Semicolon => Some(EnigoKey::Unicode(';')),
            Key::Equal => Some(EnigoKey::Unicode('=')),
            Key::Comma => Some(EnigoKey::Unicode(',')),
            Key::Minus => Some(EnigoKey::Unicode('-')),
            Key::Period => Some(EnigoKey::Unicode('.')),
            Key::Slash => Some(EnigoKey::Unicode('/')),
            Key::Backquote => Some(EnigoKey::Unicode('`')),
            Key::LeftBracket => Some(EnigoKey::Unicode('[')),
            Key::Backslash => Some(EnigoKey::Unicode('\\')),
            Key::RightBracket => Some(EnigoKey::Unicode(']')),
            Key::Quote => Some(EnigoKey::Unicode('\'')),
            #[cfg(target_os = "windows")]
            Key::Numpad0 => Some(EnigoKey::Numpad0),
            #[cfg(target_os = "windows")]
            Key::Numpad1 => Some(EnigoKey::Numpad1),
            #[cfg(target_os = "windows")]
            Key::Numpad2 => Some(EnigoKey::Numpad2),
            #[cfg(target_os = "windows")]
            Key::Numpad3 => Some(EnigoKey::Numpad3),
            #[cfg(target_os = "windows")]
            Key::Numpad4 => Some(EnigoKey::Numpad4),
            #[cfg(target_os = "windows")]
            Key::Numpad5 => Some(EnigoKey::Numpad5),
            #[cfg(target_os = "windows")]
            Key::Numpad6 => Some(EnigoKey::Numpad6),
            #[cfg(target_os = "windows")]
            Key::Numpad7 => Some(EnigoKey::Numpad7),
            #[cfg(target_os = "windows")]
            Key::Numpad8 => Some(EnigoKey::Numpad8),
            #[cfg(target_os = "windows")]
            Key::Numpad9 => Some(EnigoKey::Numpad9),
            //Key::NumpadMultiply => Some(EnigoKey::NumpadMultiply),
            //Key::NumpadAdd => Some(EnigoKey::NumpadAdd),
            //Key::NumpadSubtract => Some(EnigoKey::NumpadSubtract),
            //Key::NumpadDecimal => Some(EnigoKey::NumpadDecimal),
            //Key::NumpadDivide => Some(EnigoKey::NumpadDivide),
            //Key::NumpadEnter => Some(EnigoKey::NumpadEnter),
            Key::Meta => Some(EnigoKey::Meta),
            //Key::ContextMenu => Some(EnigoKey::Menu),
            _ => None,
        }
    }
}