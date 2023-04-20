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
	let z=app_context.compute_v_px(Dp(24.0));

	let w=app_context.text_renderer.borrow().get_str_width(&*p.text,z);
	let w2=app_context.compute_h_px(Dp(8.0));

	let c=match *hover_state {
		HoverState::Normal=>0x808080,
		HoverState::Active=>0x404040,
		HoverState::Hover=>0xa0a0a0
	};

	apx!{
		<flow width=Px(w+w2*2.0) height=Dp(48.0)>
			<blk top=Dp(8.0) bottom=Dp(8.0)>
				<bg col=c/>
				<text size=Dp(24.0) text=p.text/>
			</blk>
			<interaction hover_state_ref=hover_state on_click_option=p.on_click/>
		</flow>
		<flow width=Dp(8.0)/>
	}
}

#[main_window]
fn main()->Elements {
	let img_src=use_state(||ImageSource::from_memory(
		include_bytes!("../assets/rustacean.png")
	));

	let scale_mode=use_state(||ScaleMode::Fit);
	let align=use_state(||Align::Center);
	let valign=use_state(||VAlign::Middle);

	apx!{
		<blk top=Pc(0.0) height=Dp(48.0)>
			<bg col=0x404080/>
			<flow width=Dp(8.0)/>

			<flow_button text="Fit".to_string()
				on_click=rc_with_clone!([scale_mode],move||scale_mode.set(ScaleMode::Fit))/>
			<flow_button text="Fill".to_string()
				on_click=rc_with_clone!([scale_mode],move||scale_mode.set(ScaleMode::Fill))/>
			<flow_button text="No Scale".to_string()
				on_click=rc_with_clone!([scale_mode],move||scale_mode.set(ScaleMode::None))/>

			<flow_button text="Left".to_string()
				on_click=rc_with_clone!([align],move||align.set(Align::Left))/>
			<flow_button text="Center".to_string()
				on_click=rc_with_clone!([align],move||align.set(Align::Center))/>
			<flow_button text="Right".to_string()
				on_click=rc_with_clone!([align],move||align.set(Align::Right))/>

			<flow_button text="Top".to_string()
				on_click=rc_with_clone!([valign],move||valign.set(VAlign::Top))/>
			<flow_button text="Middle".to_string()
				on_click=rc_with_clone!([valign],move||valign.set(VAlign::Middle))/>
			<flow_button text="Bottom".to_string()
				on_click=rc_with_clone!([valign],move||valign.set(VAlign::Bottom))/>
		</blk>

		<blk width=Pc(50.0) height=Pc(50.0)>
			<bg col=0x000080/>
			<img src=img_src.as_rc()
				align=(*align).clone()
				valign=(*valign).clone()
				scale_mode=(*scale_mode).clone()/>
		</blk>
		<blk bottom=Pc(0.0) height=Dp(16.0)>
			<text text="Hello, testing image component. Putting this text to make sure the font texture doesn't get messed up.".to_string() size=Pc(100.0)/>
		</blk>
	}
}
