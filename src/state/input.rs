use rustc_hash::FxHashMap;
use winit::keyboard::KeyCode;

type Key = &'static str;

#[non_exhaustive]
#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Button {
    Key(KeyCode),
}

#[derive(Default, Debug)]
pub struct Action {
    down: bool,
}

impl Action {
    pub fn is_down(&self) -> bool {
        self.down
    }
}

#[derive(Default)]
pub struct Input {
    bindings: FxHashMap<Button, Key>,
    actions: FxHashMap<Key, Action>,
    /// A reference to this is returned when `get_action` is called on an action
    /// that does not exist.
    dummy_action: Action,
}

/// Yes
impl Input {
    /// Create an action.
    pub fn create_action(&mut self, key: Key) {
        let result = self.actions.insert(key, Action::default());
        if result.is_some() {
            log::warn!("Action {:?} created more than once", key);
        }
    }

    /// Bind a `Button` to an action.
    ///
    /// Note that each `Button` can only be bound to one action.
    pub fn create_binding(&mut self, button: Button, key: Key) {
        self.bindings.insert(button, key);
    }

    /// This is shorthand for calling `create_action` and `create_binding` subsequently.
    #[inline]
    pub fn create_action_and_binding(&mut self, key: Key, button: Button) {
        self.create_action(key);
        self.create_binding(button, key);
    }

    pub fn update_button(&mut self, button: &Button, down: bool) {
        if let Some(key) = self.bindings.get(button) {
            if let Some(action) = self.actions.get_mut(key) {
                // Ignore repeated events by making sure the new value of `down` is different.
                if action.down != down {
                    action.down = down;
                    log::trace!("Action {:?} updated\n{:?}", key, action);
                }
            } else {
                log::warn!("Button {:?} is bound to invalid action {:?}", button, key);
            }
        } else {
            // Ignore buttons that aren't bound.
        }
    }
}

impl Input {
    pub fn get_action(&self, action: Key) -> &Action {
        if let Some(action) = self.actions.get(action) {
            action
        } else {
            log::warn!("Action {:?} does not exist", action);
            &self.dummy_action
        }
    }
}
