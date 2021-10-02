use hecs::World;

pub trait KeyboardInput {
    fn on_a(&mut self, world: &mut World);
    fn on_b(&mut self, world: &mut World);
    fn on_c(&mut self, world: &mut World);
    fn on_d(&mut self, world: &mut World);
    fn on_e(&mut self, world: &mut World);
    fn on_f(&mut self, world: &mut World);
    fn on_g(&mut self, world: &mut World);
    fn on_h(&mut self, world: &mut World);
    fn on_i(&mut self, world: &mut World);
    fn on_j(&mut self, world: &mut World);
    fn on_k(&mut self, world: &mut World);
    fn on_l(&mut self, world: &mut World);
    fn on_m(&mut self, world: &mut World);
    fn on_n(&mut self, world: &mut World);
    fn on_o(&mut self, world: &mut World);
    fn on_p(&mut self, world: &mut World);
    fn on_q(&mut self, world: &mut World);
    fn on_r(&mut self, world: &mut World);
    fn on_s(&mut self, world: &mut World);
    fn on_t(&mut self, world: &mut World);
    fn on_u(&mut self, world: &mut World);
    fn on_v(&mut self, world: &mut World);
    fn on_w(&mut self, world: &mut World);
    fn on_x(&mut self, world: &mut World);
    fn on_y(&mut self, world: &mut World);
    fn on_z(&mut self, world: &mut World);
}
