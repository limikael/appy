/*use std::any::Any;
use std::rc::Rc;
use crate::utils::{FlowBucket, FlowConf};
use crate::types::{Align, VAlign};

#[test]
fn test() {
    let mut fb=FlowBucket::<String>::new(FlowConf{
        width: 100.0,
        height: 100.0,
        gap: 5.0,
        vgap: 5.0,
        align: Align::Left,
        valign: VAlign::Top
    });

    fb.add("hello".to_string(),45.0,10.0);
    fb.add("worldo".to_string(),45.0,10.0);
    fb.add("bla".to_string(),45.0,10.0);

    println!("{:?}",fb);

    println!("{:?}",fb.height());

    fb.with_items(|item,x,y,w,h|{
        println!("{:?} {},{} {},{}",item,x,y,w,h);
    })
}
*/
