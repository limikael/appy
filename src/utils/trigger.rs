use crate::*;
use std::cell::RefCell;
use std::rc::Rc;

/// A trigger is similar to an event, but more asynchronous in nature.
///
/// A trigger can be created in one place, and then a trigger function can
/// be created. The trigger function can be passed to another piece of code,
/// and that piece of code can call the function to trigger the trigger.
///
/// When the trigger is triggered, no handler will be called (as with an event).
/// Instead, a state will be set, which can later be checked where the trigger
/// was created.
pub struct Trigger {
    state: Rc<RefCell<bool>>,
    trigger: Rc<dyn Fn()>,
}

impl Trigger {
    /// Create a trigger.
    pub fn new() -> Self {
        let state = Rc::new(RefCell::new(false));
        let trigger = Rc::new(with_clone!([state], move || {
            *state.borrow_mut() = true;
        }));

        Trigger { state, trigger }
    }

    /// Get the current state of the trigger.
    pub fn get_state(&self) -> bool {
        *self.state.borrow()
    }

    /// Set the current state of the trigger.
    pub fn set_state(&self, new_state: bool) {
        *self.state.borrow_mut() = new_state;
    }

    /// Get a trigger function for this trigger.
    ///
    /// This trigger function can be safely passed to another part of
    /// the program, as well as cheaply cloned.
    pub fn create_trigger(&self) -> Rc<dyn Fn()> {
        self.trigger.clone()
    }
}
