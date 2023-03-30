use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct Trigger {
    pub state: Rc<RefCell<bool>>,
}

impl Trigger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_state(&self) -> bool {
        *self.state.borrow()
    }

    pub fn set_state(&self, new_state: bool) {
        *self.state.borrow_mut() = new_state;
    }

    pub fn create_trigger(&self) -> Rc<dyn Fn()> {
        let state_rc = self.state.clone();
        Rc::new(move || {
            *state_rc.borrow_mut() = true;
        })
    }
}
