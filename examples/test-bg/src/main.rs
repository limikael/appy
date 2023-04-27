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

	let p=(*val/max)*(app_context.rect.w as f32-app_context.compute_h_px(Dim::DeviceIndependentPixels(20.0)));

	apx!{
		<blk top=0 height=20>
			<text text=&*props.label size=pct(100) />
		</blk>
		<blk top=25 height=20>
			<blk width=pct(100) height=10>
				<bg color=0x808080 corner_radius=5 border_width=1 border_color=0xffffff/>
			</blk>
			<blk width=20 height=20 left=Dim::HardwarePixels(p)>
				<bg color=0xc0c0c0 border_width=1 border_color=0xffffff corner_radius=10/>
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
		<bg border_width=2 border_color=0xc0c0c0 color=0x000080 corner_radius=5/>
		{if *val {
			apx!{
				<blk margin=5>
					<bg corner_radius=5 color=0xc0c0c0/>
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
		<blk left=0 width=200>
			<bg color=0x000080/>

			<blk margin=20>
				<flow width=pct(100) height=60>
					<slider value=rad.clone() max=50.0
							label=&*format!("Rad: {}",*rad) />
				</flow>
				<flow width=pct(100) height=60>
					<slider value=border.clone() max=50.0
							label=&*format!("Border: {}",*border)/>
				</flow>
				<flow width=pct(100) height=60>
					<blk top=0 height=20>
						<text text="Borders" size=pct(100) />
					</blk>
					<blk bottom=0 height=40>
						<grid cols=4>
							{borders.iter().flat_map(move|b|{
								apx!{
									<blk margin=5>
										<check_box value=b.clone()/>
									</blk>
								}
							}).collect()}
						</grid>
					</blk>
				</flow>
			</blk>
		</blk>
		<blk left=250 top=100 width=400 height=200>
			<bg color=0x000080
				corner_radius=*rad
				border_color=0x8080ff
				border_width=*border
				borders=borders.map(|b|(*b).clone())/>
		</blk>
	}
}
