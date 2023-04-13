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

pub trait AppWindow {
    fn run(self: Box<Self>, handler:Box<dyn FnMut(&mut dyn AppWindow,AppEvent)>);
    fn post_redisplay(&mut self);
    fn size(&self)->(i32,i32);
    fn pixel_ratio(&self)->f32;
}

pub trait AppWindowBuilder {
    fn build(&mut self)->Box<dyn AppWindow>;
}