use crate::events::EventQueue;
use std::fmt::Debug;

pub trait Physics: Debug {
    fn collision(&mut self, event_queue: &mut EventQueue);
}

pub trait Input: Debug {
    fn input(&mut self, event_queue: &mut EventQueue);
}

pub trait Logic: Debug {
    fn update(&mut self, event_queue: &mut EventQueue);
}

pub trait Draw: Debug {
    fn draw(&mut self, event_queue: &mut EventQueue);
}

pub trait Interface: Debug {
    fn draw_interface(&mut self, event_queue: &mut EventQueue);
}
