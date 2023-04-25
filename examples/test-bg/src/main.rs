use appy::{*,types::*,components::*, hooks::*};
use glapp::{AppEvent};

#[derive_component(ComponentBuilder,Default,SnakeFactory)]
pub struct Slider {
	label: String,
	value: Option<StateRef<f32>>,
	max: f32
}

#[function_component]
fn _slider(props:Slider)->Elements {
	let mut val=use_state(||0.0);
	if props.value.is_some() {
		val=props.value.unwrap();
	}

	let down=use_state(||false);
	let app_context=use_context::<AppContext>();
	let max=props.max;

	use_app_event(rc_with_clone!([val,app_context,down,max],move|e|{
		let update=rc_with_clone!([val,app_context],move|x:i32| {
			let v=max*(x-app_context.rect.x) as f32/app_context.rect.w as f32;
			val.set(v.max(0.0).min(max));
		});

		match *e {
			AppEvent::MouseDown{x,y,..}=>{
				if app_context.rect.contains(x,y) {
					update(x);
					down.set(true);
				}
			}
			AppEvent::MouseMove{x,..}=>{
				if *down {
					update(x);
				}
			}
			AppEvent::MouseUp{..}=>{
				down.set(false);
			}
            _ => {}
		}
	}));

	let p=(*val/max)*(app_context.rect.w as f32-app_context.compute_h_px(Dp(20.0)));

	apx!{
		<blk top=Dp(0.0) height=Dp(20.0)>
			<text text=props.label size=Pc(100.0) />
		</blk>
		<blk top=Dp(25.0) height=Dp(20.0)>
			<blk width=Pc(100.0) height=Dp(10.0)>
				<bg color=0x808080 corner_radius=Dp(5.0) border_width=Dp(1.0) border_color=0xffffff/>
			</blk>
			<blk width=Dp(20.0) height=Dp(20.0) left=Px(p)>
				<bg color=0xc0c0c0 border_width=Dp(1.0) border_color=0xffffff corner_radius=Dp(10.0)/>
			</blk>
		</blk>
	}	
}

#[derive_component(ComponentBuilder,Default,SnakeFactory)]
pub struct CheckBox {
	value: Option<StateRef<bool>>,
}

#[function_component]
fn _check_box(props:CheckBox)->Elements {
	let mut val=use_state(||false);
	if props.value.is_some() {
		val=props.value.unwrap();
	}

	apx!{
		<bg border_width=Dp(2.0) border_color=0xc0c0c0 color=0x000080 corner_radius=Dp(5.0)/>
		{if *val {
			apx!{
				<blk margin=Dp(5.0)>
					<bg corner_radius=Dp(5.0) color=0xc0c0c0/>
				</blk>
			}
		} else {apx!{}}}

		<interaction on_click=rc_with_clone!([val],move||val.set(!*val))/>
	}
}

#[main_window]
fn main()->Elements {
	let rad=use_state(||10.0);
	let border=use_state(||5.0);

	let borders=[
		use_state(||true),
		use_state(||true),
		use_state(||true),
		use_state(||true)
	];

	apx!{
		<blk left=Dp(0.0) width=Dp(200.0)>
			<bg color=0x000080/>

			<blk margin=Dp(20.0)>
				<flow width=Pc(100.0) height=Dp(60.0)>
					<slider value=rad.clone() max=50.0
							label=format!("Rad: {}",*rad).to_string() />
				</flow>
				<flow width=Pc(100.0) height=Dp(60.0)>
					<slider value=border.clone() max=50.0
							label=format!("Border: {}",*border).to_string()/>
				</flow>
				<flow width=Pc(100.0) height=Dp(60.0)>
					<blk top=Dp(0.0) height=Dp(20.0)>
						<text text="Borders".to_string() size=Pc(100.0) />
					</blk>
					<blk bottom=Dp(0.0) height=Dp(40.0)>
						<grid cols=4>
							{borders.iter().flat_map(move|b|{
								apx!{
									<blk margin=Dp(5.0)>
										<check_box value=b.clone()/>
									</blk>
								}
							}).collect()}
						</grid>
					</blk>
				</flow>
			</blk>
		</blk>
		<blk left=Dp(250.0) top=Dp(100.0) width=Dp(400.0) height=Dp(200.0)>
			<bg color=0x000080
				corner_radius=Dp(*rad)
				border_color=0x8080ff
				border_width=Dp(*border)
				borders=borders.map(|b|(*b).clone())/>
		</blk>
	}
}
