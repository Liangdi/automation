use std::time::Instant;

use crate::{enums::InputAction, simulator::InputSimulator};

/// 动作执行器
pub struct ActionExecutor {
    simulator: InputSimulator,
}

impl ActionExecutor {
    /// 创建新的动作执行器
    pub fn new() -> Self {
        ActionExecutor {
            simulator: InputSimulator::new(),
        }
    }
    
    /// 执行单个动作并返回结果和耗时
    pub async fn execute(&self, action: &InputAction) -> (String, u128) {
        let start_time = Instant::now();
        let result = self.simulator.execute_action(action).await;
        let duration = start_time.elapsed().as_millis();
        (result, duration)
    }
    
    /// 获取屏幕尺寸
    pub fn get_screen_size(&self) -> (u32, u32) {
        self.simulator.get_screen_size()
    }
    
    /// 获取底层模拟器引用
    pub fn simulator(&self) -> &InputSimulator {
        &self.simulator
    }
}