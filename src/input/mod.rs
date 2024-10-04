use rustc_hash::FxHashMap;

type Key = &'static str;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Button {
    Key(winit::keyboard::KeyCode),
    Mouse(winit::event::MouseButton),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Action {
    pub is_down: bool,
    pub just_down: bool,
    pub just_up: bool,
}

#[derive(Default)]
pub struct Input {
    bindings: FxHashMap<Button, Key>,
    actions: FxHashMap<Key, Action>,
    mouse_delta: glam::Vec2,
}

impl Input {
    /// Register a new action.
    pub fn register_action(&mut self, key: Key) {
        let result = self.actions.insert(key, Action::default());

        if result.is_some() {
            log::warn!("Action {:?} was already registered", key);
        }

        log::debug!("Registered action {:?}", key);
    }

    /// Bind a button to an action.
    pub fn bind_button(&mut self, button: Button, key: Key) {
        if self.bindings.contains_key(&button) {
            log::warn!("Button {:?} was already bound", button);
        }

        self.bindings.insert(button, key);
    }

    /// Update the state of the action that the button is bound to.
    ///
    /// If the button is not bound to an action, nothing will happen.
    pub fn update_button(&mut self, button: Button, is_down: bool) {
        if let Some(key) = self.bindings.get(&button) {
            self.update_action(key, is_down);
        }
    }

    /// Update the state of an action using the action's key.
    pub fn update_action(&mut self, key: Key, is_down: bool) {
        if let Some(action) = self.actions.get_mut(key) {
            if action.is_down != is_down {
                action.is_down = is_down;
                action.just_down = is_down;
                action.just_up = !is_down;

                log::debug!("{:?} updated\n{:?}", key, action);
            }
        } else {
            log::warn!("Action {:?} was not registered", key);
        }
    }

    /// Get the state of an action.
    pub fn get_action(&self, key: Key) -> Action {
        if let Some(action) = self.actions.get(key).copied() {
            action
        } else {
            log::warn!("Action {:?} was not registered", key);
            Action::default()
        }
    }

    pub fn update_mouse_delta(&mut self, delta: glam::Vec2) {
        self.mouse_delta += delta;
        log::debug!("`mouse_delta` updated\n{:?}", self.mouse_delta);
    }

    pub fn get_mouse_delta(&self) -> glam::Vec2 {
        self.mouse_delta
    }

    /// Update the state of all actions.
    ///
    /// This resets temporary values like `just_down` and `just_up`.
    pub fn update(&mut self) {
        for action in self.actions.values_mut() {
            action.just_down = false;
            action.just_up = false;
        }

        self.mouse_delta = glam::Vec2::ZERO;
    }
}
