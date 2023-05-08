use crate::core::component::HookRef;
use crate::core::Appy;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone)]
pub struct ReducerRef<T, A> {
    hook_ref: HookRef<T>,
    reducer: Rc<dyn Fn(&T, A) -> T>,
}

impl<T: 'static, A> ReducerRef<T, A> {
    pub fn dispatch(&self, action: A) {
        let reduced: T = (self.reducer)(&*self.hook_ref, action);
        self.hook_ref.set(reduced);
    }
}

impl<T, A> Deref for ReducerRef<T, A> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.hook_ref
    }
}

/// Similar to the `use_state` hook, but passes the data through a reducer function.
pub fn use_reducer<F, G, A: 'static, T: 'static>(reducer: G, ctor: F) -> ReducerRef<T, A>
where
    F: Fn() -> T,
    G: Fn(&T, A) -> T + 'static,
{
    Appy::with(|appy| ReducerRef {
        hook_ref: appy.use_hook_ref(ctor),
        reducer: Rc::new(reducer),
    })
}
