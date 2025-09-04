use enigo::*;
use enigo::Key as EnigoKey;
use crate::enums::InputAction;
use super::{keyboard::KeyboardSimulator, mouse::MouseSimulator};
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

/// 输入模拟器核心实现
pub struct InputSimulator {
    enigo: Arc<Mutex<Enigo>>,
    keyboard: KeyboardSimulator,
    mouse: MouseSimulator,
}

impl InputSimulator {
    /// 创建新的输入模拟器实例
    pub fn new() -> Self {
        InputSimulator {
            enigo: Arc::new(Mutex::new(Enigo::new(&Settings::default()).unwrap())),
            keyboard: KeyboardSimulator::new(),
            mouse: MouseSimulator::new(),
        }
    }

    /// 获取屏幕尺寸
    pub fn get_screen_size(&self) -> (u32, u32) {
        
        (0,0)
    }

    /// 执行单个输入动作
    pub async fn execute_action(&self, action: &InputAction) -> String {
        let mut enigo = self.enigo.lock().unwrap();
        self.execute_with_enigo(&mut enigo, action).await
    }

    /// 使用指定的Enigo实例执行动作
    pub async fn execute_with_enigo(&self, enigo: &mut Enigo, action: &InputAction) -> String {
        match action {
            // 鼠标操作
            InputAction::MouseClick { button, x, y } => {
                self.mouse.click(enigo, *button, *x, *y);
                format!("Clicked {:?} at ({}, {})", button, x, y)
            }
            InputAction::MouseDoubleClick { button, x, y } => {
                self.mouse.double_click(enigo, *button, *x, *y);
                format!("Double clicked {:?} at ({}, {})", button, x, y)
            }
            InputAction::MouseMove { x, y } => {
                self.mouse.move_to(enigo, *x, *y);
                format!("Moved to ({}, {})", x, y)
            }
            InputAction::MouseScroll { delta_x, delta_y } => {
                self.mouse.scroll(enigo, *delta_x, *delta_y);
                format!("Scrolled: horizontal {}, vertical {}", delta_x, delta_y)
            }
            InputAction::MousePress { button, x, y, duration_ms } => {
                self.mouse.press(enigo, *button, *x, *y, *duration_ms).await;
                format!("Pressed {:?} at ({}, {}) for {}ms", button, x, y, duration_ms)
            }
            InputAction::MouseDrag { button, start_x, start_y, end_x, end_y, duration_ms } => {
                self.mouse.drag(
                    enigo, 
                    *button, 
                    *start_x, 
                    *start_y, 
                    *end_x, 
                    *end_y, 
                    *duration_ms
                ).await;
                format!(
                    "Dragged {:?} from ({}, {}) to ({}, {}) over {}ms", 
                    button, start_x, start_y, end_x, end_y, duration_ms
                )
            }
            
            // 键盘操作
            InputAction::KeyPress { key, location } => {
                self.keyboard.press_key(enigo, *key, *location);
                format!("Pressed key {:?}", key)
            }
            InputAction::KeyDown { key, location } => {
                self.keyboard.key_down(enigo, *key, *location);
                format!("Key down: {:?}", key)
            }
            InputAction::KeyUp { key, location } => {
                self.keyboard.key_up(enigo, *key, *location);
                format!("Key up: {:?}", key)
            }
            InputAction::KeySequence { keys, key_delay_ms } => {
                for (i, key) in keys.iter().enumerate() {
                    self.keyboard.press_key(enigo, *key, None);
                    if let Some(delay) = key_delay_ms {
                        if i < keys.len() - 1 {
                            sleep(Duration::from_millis(*delay)).await;
                        }
                    }
                }
                format!("Pressed sequence of {} keys", keys.len())
            }
            InputAction::TypeText { text, char_delay_ms } => {
                if let Some(delay) = char_delay_ms {
                    for c in text.chars() {
                        let _ = enigo.key(EnigoKey::Unicode(c),Direction::Press);
                        sleep(Duration::from_millis(*delay)).await;
                    }
                } else {
                    let _ = enigo.text(text);
                }
                format!("Typed text: {}", text)
            }
            InputAction::Hotkey { modifiers, key, location } => {
                self.keyboard.hotkey(enigo, modifiers, *key, *location);
                format!("Pressed hotkey: modifiers {:?} + {:?}", modifiers, key)
            }
            
            // 延时操作
            InputAction::Delay { milliseconds } => {
                sleep(Duration::from_millis(*milliseconds)).await;
                format!("Delayed for {}ms", milliseconds)
            }
            
            // 组合操作
            InputAction::Sequence { actions } => {
                let mut results = Vec::new();
                for (i, action) in actions.iter().enumerate() {
                    let result = Box::pin(self.execute_with_enigo(enigo, action)).await;
                    results.push(format!("Step {}: {}", i + 1, result));
                }
                format!("Sequence completed:\n{}", results.join("\n"))
            }
            InputAction::Parallel { actions } => {
                //TODO
                //let mut tasks = Vec::new();
                //for action in actions.iter() {
                //    let action_clone = action.clone();
                //    let enigo_clone = self.enigo.clone();
                //    let simulator_clone = self.clone();
                    
                    // tasks.push(tokio::spawn(async move {
                    //     let mut enigo = enigo_clone.lock().unwrap();
                    //     simulator_clone.execute_with_enigo(&mut enigo, &action_clone).await
                    // }));
                //}
                
                // let mut results = Vec::new();
                // for (i, task) in tasks.into_iter().enumerate() {
                //     let result = task.await.unwrap();
                //     results.push(format!("Task {}: {}", i + 1, result));
                // }
                
                format!("Parallel actions completed")
            }
        }
    }
}

impl Clone for InputSimulator {
    fn clone(&self) -> Self {
        InputSimulator {
            enigo: self.enigo.clone(),
            keyboard: KeyboardSimulator::new(),
            mouse: MouseSimulator::new(),
        }
    }
}