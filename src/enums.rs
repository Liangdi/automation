use serde::{Deserialize, Serialize};

/// 鼠标按键枚举
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,   // 浏览器后退按钮
    Forward, // 浏览器前进按钮
    Other(u8),
}

/// 键盘按键位置（用于区分左右修饰键）
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum KeyLocation {
    Standard, // 标准位置
    Left,     // 左侧修饰键
    Right,    // 右侧修饰键
    Numpad,   // 数字小键盘
}

/// 键盘按键枚举（扩展）
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Key {
    Backspace,
    Tab,
    Enter,
    Shift,
    Ctrl,
    Alt,
    CapsLock,
    Escape,
    Space,
    PageUp,
    PageDown,
    End,
    Home,
    ArrowLeft,
    ArrowUp,
    ArrowRight,
    ArrowDown,
    PrintScreen,
    Insert,
    Delete,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Semicolon,   // ;
    Equal,       // =
    Comma,       // ,
    Minus,       // -
    Period,      // .
    Slash,       // /
    Backquote,   // `
    LeftBracket, // [
    Backslash,   // \
    RightBracket,// ]
    Quote,       // '
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadMultiply,
    NumpadAdd,
    NumpadSubtract,
    NumpadDecimal,
    NumpadDivide,
    NumpadEnter,
    Meta, // Windows键或Command键
    ContextMenu,
    VolumeMute,
    VolumeDown,
    VolumeUp,
    MediaPlayPause,
    MediaStop,
    MediaNextTrack,
    MediaPreviousTrack,
    Other(u32), // 其他键
}

/// 输入操作枚举
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "params")]
pub enum InputAction {
    // 鼠标操作
    MouseMove {
        x: i32,
        y: i32,
    },
    MouseClick {
        button: MouseButton,
        x: i32,
        y: i32,
    },
    MouseDoubleClick {
        button: MouseButton,
        x: i32,
        y: i32,
    },
    MousePress {
        button: MouseButton,
        x: i32,
        y: i32,
        duration_ms: u64,
    },
    MouseDrag {
        button: MouseButton,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        duration_ms: u64,
    },
    MouseScroll {
        delta_x: i32,
        delta_y: i32,
    },
    
    // 键盘操作
    KeyPress {
        key: Key,
        location: Option<KeyLocation>,
    },
    KeyDown {
        key: Key,
        location: Option<KeyLocation>,
    },
    KeyUp {
        key: Key,
        location: Option<KeyLocation>,
    },
    KeySequence {
        keys: Vec<Key>,
        key_delay_ms: Option<u64>,
    },
    TypeText {
        text: String,
        char_delay_ms: Option<u64>,
    },
    Hotkey {
        modifiers: Vec<Key>,
        key: Key,
        location: Option<KeyLocation>,
    },
    
    // 延时操作
    Delay {
        milliseconds: u64,
    },
    
    // 组合操作
    Sequence {
        actions: Vec<InputAction>,
    },
    Parallel {
        actions: Vec<InputAction>,
    },
}