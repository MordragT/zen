use std::any::{Any, TypeId};

#[derive(PartialEq, Eq, Debug)]
pub enum Lifetime {
    AfterPhysics,
    AfterInput,
    AfterLogic,
    AfterDraw,
    AfterDrawInterface,
    Never,
}

#[derive(Default, Debug)]
pub struct EventQueue {
    active_events: Vec<(Box<dyn Any>, Lifetime)>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.active_events.clear();
    }

    pub fn clear_solved(&mut self, current: Lifetime) {
        self.active_events.drain_filter(|event| event.1 == current);
    }

    pub fn send(&mut self, event: Box<dyn Any>, lifetime: Lifetime) {
        self.active_events.push((event, lifetime));
    }

    pub fn event_triggered<E: 'static>(&self) -> bool {
        for event in self.active_events.iter() {
            if (&*event.0).type_id() == TypeId::of::<E>() {
                return true;
            }
        }
        false
    }
}
