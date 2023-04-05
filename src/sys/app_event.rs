#[derive(Debug)]
pub enum MouseButton {
	Left,
	Right,
	Unknown
}

#[derive(Debug)]
pub enum MouseKind {
	Mouse,
	Touch
}

#[derive(Debug)]
pub enum AppEvent {
    Show,
    Render,
    Resize{width:u32, height:u32},
    MouseDown{x:i32, y:i32, kind:MouseKind, button:MouseButton},
    MouseUp{x:i32, y:i32, kind:MouseKind, button:MouseButton},
    MouseMove{x:i32, y:i32, kind:MouseKind},
}
