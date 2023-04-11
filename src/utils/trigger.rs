use std::cell::RefCell;
use std::rc::Rc;
use crate::*;

pub struct Trigger {
    state: Rc<RefCell<bool>>,
    trigger: Rc<dyn Fn()>
}

impl Trigger {
    pub fn new() -> Self {
        let state=Rc::new(RefCell::new(false));
        let trigger=Rc::new(with_clone!([state],move||{
            *state.borrow_mut() = true;
        }));

        Trigger {
            state,
            trigger
        }
    }

    pub fn get_state(&self) -> bool {
        *self.state.borrow()
    }

    pub fn set_state(&self, new_state: bool) {
        *self.state.borrow_mut() = new_state;
    }

    pub fn create_trigger(&self) -> Rc<dyn Fn()> {
        self.trigger.clone()
    }
}
