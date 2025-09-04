use enigo::*;
use crate::enums::{Key, KeyLocation};
use super::keymap::KeyMapper;

/// 键盘模拟器实现
pub struct KeyboardSimulator {
    key_mapper: KeyMapper,
}

impl KeyboardSimulator {
    pub fn new() -> Self {
        KeyboardSimulator {
            key_mapper: KeyMapper::new(),
        }
    }

    /// 按下并释放单个键
    pub fn press_key(&self, enigo: &mut Enigo, key: Key, location: Option<KeyLocation>) {
        self.key_down(enigo, key, location.clone());
        self.key_up(enigo, key, location);
    }

    /// 按下键（不释放）
    pub fn key_down(&self, enigo: &mut Enigo, key: Key, location: Option<KeyLocation>) {
        if let Some(key_code) = self.key_mapper.map_key(key, location) {
            enigo.key(key_code, Direction::Press);
        }
    }

    /// 释放键
    pub fn key_up(&self, enigo: &mut Enigo, key: Key, location: Option<KeyLocation>) {
        if let Some(key_code) = self.key_mapper.map_key(key, location) {
            enigo.key(key_code, Direction::Release);
        }
    }

    /// 输入文本
    pub fn type_text(&self, enigo: &mut Enigo, text: &str) {
        enigo.text(text);
    }

    /// 执行热键组合
    pub fn hotkey(
        &self, 
        enigo: &mut Enigo, 
        modifiers: &[Key], 
        key: Key, 
        location: Option<KeyLocation>
    ) {
        // 按下所有修饰键
        for modifier in modifiers {
            self.key_down(enigo, *modifier, None);
        }
        
        // 按下并释放主键
        self.press_key(enigo, key, location);
        
        // 释放所有修饰键
        for modifier in modifiers.iter().rev() {
            self.key_up(enigo, *modifier, None);
        }
    }
}