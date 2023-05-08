use appy::{*, components::*, hooks::*, types::*};
use std::rc::Rc;

#[derive_component(ComponentBuilder,Default,SnakeFactory)]
pub struct FlowButton {
	text: String,
	on_click: Option<Rc<dyn Fn()>>
}

#[function_component]
fn _flow_button(p:FlowButton)->Elements {
	let hover_state=use_hover_state_ref();
	let app_context=use_context::<AppContext>();
	let w=app_context.default_font.get_str_width(&*p.text,24.0);

	let c=match *hover_state {
		HoverState::Normal=>0x808080,
		HoverState::Active=>0x404040,
		HoverState::Hover=>0xa0a0a0
	};

	apx!{
		<flow width=w+16.0 height=48>
			<blk top=8 bottom=8>
				<bg color=c border_width=1 border_color=0xffffff/>
				<text size=24 text=&*p.text/>
			</blk>
			<interaction hover_state_ref=hover_state on_click_option=p.on_click/>
		</flow>
	}
}

#[main_window]
fn main()->Elements {
	let img_src=use_state(||ImageSource::from_memory(
		include_bytes!("rustacean.png")
	));

	let scale_mode=use_state(||ScaleMode::Fit);
	let align=use_state(||Align::Center);
	let valign=use_state(||VAlign::Middle);

	apx!{
		<blk top=0 height=48 flow_gap=8>
			<bg color=0x404080/>
			<flow_button text="Fit"
				on_click=rc_with_clone!([scale_mode],move||scale_mode.set(ScaleMode::Fit))/>
			<flow_button text="Fill"
				on_click=rc_with_clone!([scale_mode],move||scale_mode.set(ScaleMode::Fill))/>
			<flow_button text="No Scale"
				on_click=rc_with_clone!([scale_mode],move||scale_mode.set(ScaleMode::None))/>

			<flow_button text="Left"
				on_click=rc_with_clone!([align],move||align.set(Align::Left))/>
			<flow_button text="Center"
				on_click=rc_with_clone!([align],move||align.set(Align::Center))/>
			<flow_button text="Right"
				on_click=rc_with_clone!([align],move||align.set(Align::Right))/>

			<flow_button text="Top"
				on_click=rc_with_clone!([valign],move||valign.set(VAlign::Top))/>
			<flow_button text="Middle"
				on_click=rc_with_clone!([valign],move||valign.set(VAlign::Middle))/>
			<flow_button text="Bottom"
				on_click=rc_with_clone!([valign],move||valign.set(VAlign::Bottom))/>
		</blk>

		<blk width=pct(50) height=pct(50)>
			<bg color=0x000080/>
			<img src=img_src.as_rc()
				align=(*align).clone()
				valign=(*valign).clone()
				scale_mode=(*scale_mode).clone()/>
		</blk>
		<blk bottom=0 height=16>
			<text text="Hello, testing image component. Putting this text to make sure the font texture doesn't get messed up." size=pct(100)/>
		</blk>
	}
}
