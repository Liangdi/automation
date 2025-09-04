use enigo::*;
use crate::enums::MouseButton;
use tokio::time::{sleep, Duration};

/// 鼠标模拟器实现
pub struct MouseSimulator;

impl MouseSimulator {
    pub fn new() -> Self {
        MouseSimulator
    }

    /// 移动鼠标到指定位置
    pub fn move_to(&self, enigo: &mut Enigo, x: i32, y: i32) {
        enigo.move_mouse(x, y,Coordinate::Abs);
    }

    /// 单击鼠标按钮
    pub fn click(&self, enigo: &mut Enigo, button: MouseButton, x: i32, y: i32) {
        self.move_to(enigo, x, y);
        enigo.button(button.into(),Direction::Click);
    }

    /// 双击鼠标按钮
    pub fn double_click(&self, enigo: &mut Enigo, button: MouseButton, x: i32, y: i32) {
        self.move_to(enigo, x, y);
        enigo.button(button.into(),Direction::Click);
        // delay
        enigo.button(button.into(),Direction::Click);
    }

    /// 按下鼠标按钮（保持）
    pub fn button_down(&self, enigo: &mut Enigo, button: MouseButton) {
        enigo.button(button.into(), Direction::Press);
    }

    /// 释放鼠标按钮
    pub fn button_up(&self, enigo: &mut Enigo, button: MouseButton) {
        enigo.button(button.into(), Direction::Release);
    }

    /// 滚动鼠标
    pub fn scroll(&self, enigo: &mut Enigo, delta_x: i32, delta_y: i32) {
        enigo.scroll(delta_x,Axis::Vertical);
        enigo.scroll(delta_y,Axis::Horizontal);
    }

    /// 长按鼠标按钮
    pub async fn press(
        &self, 
        enigo: &mut Enigo, 
        button: MouseButton, 
        x: i32, 
        y: i32, 
        duration_ms: u64
    ) {
        self.move_to(enigo, x, y);
        self.button_down(enigo, button);
        sleep(Duration::from_millis(duration_ms)).await;
        self.button_up(enigo, button);
    }

    /// 拖拽操作（带平滑移动）
    pub async fn drag(
        &self, 
        enigo: &mut Enigo, 
        button: MouseButton, 
        start_x: i32, 
        start_y: i32, 
        end_x: i32, 
        end_y: i32, 
        duration_ms: u64
    ) {
        self.move_to(enigo, start_x, start_y);
        self.button_down(enigo, button);
        
        // 平滑拖动效果
        let steps = 20;
        let step_duration = duration_ms / steps as u64;
        let dx = (end_x - start_x) as f32 / steps as f32;
        let dy = (end_y - start_y) as f32 / steps as f32;
        
        for i in 1..=steps {
            let x = start_x + (dx * i as f32) as i32;
            let y = start_y + (dy * i as f32) as i32;
            self.move_to(enigo, x, y);
            sleep(Duration::from_millis(step_duration)).await;
        }
        
        self.button_up(enigo, button);
    }
}

// 为MouseButton实现到Enigo按钮类型的转换
impl From<MouseButton> for enigo::Button {
    fn from(button: MouseButton) -> Self {
        match button {
            MouseButton::Left => enigo::Button::Left,
            MouseButton::Right => enigo::Button::Right,
            MouseButton::Middle => enigo::Button::Middle,
            MouseButton::Back => enigo::Button::Back,
            MouseButton::Forward => enigo::Button::Forward,
            _ => enigo::Button::Left,
        }
    }
}