use dino_engine::prelude::*;

#[derive(Component)]
struct Name(String);
#[derive(Component)]
struct Age(i32);
#[derive(Component)]
struct Gender(bool);

struct Test;
impl TSystem for Test {
    fn init(&mut self, world: &mut World) {
        world
            .spawn()
            .insert(Name("George".to_string()))
            .insert(Age(15))
            .insert(Gender(false));
    }
    fn update(&mut self, world: &mut World) {
        for (name, age, dead) in world.query::<(Name, Age, Gender)>().iter() {
            println!(
                "{} is {} yo and is {}",
                name.0,
                age.0,
                if dead.0 { "dead" } else { "alive" }
            );
        }
    }
}

fn main() {
    run([Box::new(Test)]);
}
