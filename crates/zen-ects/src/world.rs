use crate::components::*;
use crate::error::*;
use crate::events::{EventQueue, Lifetime};

type ComponentVec<T> = Vec<Option<Box<T>>>;

#[derive(Debug)]
pub struct World {
    entity_count: usize,
    physics: ComponentVec<dyn Physics>,
    input: ComponentVec<dyn Input>,
    logic: ComponentVec<dyn Logic>,
    draw: ComponentVec<dyn Draw>,
    interface: ComponentVec<dyn Interface>,
    event_queue: EventQueue,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    pub fn new() -> Self {
        World {
            entity_count: 0,
            physics: Vec::new(),
            input: Vec::new(),
            logic: Vec::new(),
            draw: Vec::new(),
            interface: Vec::new(),
            event_queue: EventQueue::new(),
        }
    }

    pub fn spawn(&mut self) -> usize {
        self.physics.push(Option::None);
        self.input.push(Option::None);
        self.logic.push(Option::None);
        self.draw.push(Option::None);
        self.interface.push(Option::None);

        let entity = self.entity_count;
        self.entity_count += 1;
        return entity;
    }

    pub fn add_physics(&mut self, entity: usize, component: Box<dyn Physics>) -> Result<()> {
        match self.physics.get_mut(entity) {
            Some(c) => {
                *c = Option::Some(component);
                Ok(())
            }
            None => Err(EctsError::OutOfBounds(entity)),
        }
    }

    pub fn add_input(&mut self, entity: usize, component: Box<dyn Input>) -> Result<()> {
        match self.input.get_mut(entity) {
            Some(c) => {
                *c = Option::Some(component);
                Ok(())
            }
            None => Err(EctsError::OutOfBounds(entity)),
        }
    }

    pub fn add_logic(&mut self, entity: usize, component: Box<dyn Logic>) -> Result<()> {
        match self.logic.get_mut(entity) {
            Some(c) => {
                *c = Option::Some(component);
                Ok(())
            }
            None => Err(EctsError::OutOfBounds(entity)),
        }
    }

    pub fn add_draw(&mut self, entity: usize, component: Box<dyn Draw>) -> Result<()> {
        match self.draw.get_mut(entity) {
            Some(c) => {
                *c = Option::Some(component);
                Ok(())
            }
            None => Err(EctsError::OutOfBounds(entity)),
        }
    }

    pub fn add_interface(&mut self, entity: usize, component: Box<dyn Interface>) -> Result<()> {
        match self.interface.get_mut(entity) {
            Some(c) => {
                *c = Option::Some(component);
                Ok(())
            }
            None => Err(EctsError::OutOfBounds(entity)),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            //println!("Physics");

            let mut physics_iter = self.physics.iter_mut();
            while let Some(entity) = physics_iter.next() {
                if let Some(component) = entity {
                    component.as_mut().collision(&mut self.event_queue);
                }
            }

            self.event_queue.clear_solved(Lifetime::AfterPhysics);

            //println!("Input");

            let mut input_iter = self.input.iter_mut();
            while let Some(entity) = input_iter.next() {
                if let Some(component) = entity {
                    component.as_mut().input(&mut self.event_queue);
                }
            }

            self.event_queue.clear_solved(Lifetime::AfterInput);

            //println!("Logic");

            let mut logic_iter = self.logic.iter_mut();
            while let Some(entity) = logic_iter.next() {
                if let Some(component) = entity {
                    component.as_mut().update(&mut self.event_queue);
                }
            }

            self.event_queue.clear_solved(Lifetime::AfterLogic);

            //println!("Draw");

            let mut draw_iter = self.draw.iter_mut();
            while let Some(entity) = draw_iter.next() {
                if let Some(component) = entity {
                    component.as_mut().draw(&mut self.event_queue);
                }
            }

            self.event_queue.clear_solved(Lifetime::AfterDraw);

            //println!("Interface");

            let mut interface_iter = self.interface.iter_mut();
            while let Some(entity) = interface_iter.next() {
                if let Some(component) = entity {
                    component.as_mut().draw_interface(&mut self.event_queue);
                }
            }

            self.event_queue.clear_solved(Lifetime::AfterDrawInterface);

            //println!("END\n\n");
            //println!("World: {:?}\n\n", &self);
            //std::thread::sleep(std::time::Duration::from_millis(2000));
        }
    }
}
