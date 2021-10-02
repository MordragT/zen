use hecs::World;

pub trait MouseInput {
    fn on_left_click(&mut self, world: &mut World);
    fn on_right_click(&mut self, world: &mut World);
}
