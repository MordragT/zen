use zen_ects::components::{Draw, Logic};
use zen_ects::events::{EventQueue, Lifetime};
use zen_ects::World;

struct NameDrawnEvent;

#[derive(Debug)]
struct Name {
    name: &'static str,
    drawn: bool,
}

impl Draw for Name {
    fn draw(&mut self, queue: &mut EventQueue) {
        if !self.drawn {
            self.drawn = true;
            println!("{}", self.name);
            queue.send(Box::new(NameDrawnEvent {}), Lifetime::AfterLogic);
        }
    }
}

#[derive(Debug)]
struct React(bool);

impl Logic for React {
    fn update(&mut self, queue: &mut EventQueue) {
        if queue.event_triggered::<NameDrawnEvent>() {
            println!("REACTION!!!");
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Logic2 {}

impl Logic for Logic2 {
    fn update(&mut self, queue: &mut EventQueue) {
        println!("as it should");
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut world = World::new();

    let player = world.spawn();
    let name = Name {
        name: "player",
        drawn: false,
    };
    let logic = Logic2 {};
    world.add_draw(player, Box::new(name))?;
    world.add_logic(player, Box::new(logic.clone()))?;

    let other = world.spawn();
    let react = React(false);
    let name2 = Name {
        name: "other",
        drawn: false,
    };
    world.add_logic(other, Box::new(react))?;
    world.add_draw(other, Box::new(name2))?;

    world.run()?;

    Ok(())
}
