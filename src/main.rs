mod vector2;
mod entity;
mod system;
mod component;
mod physics;
mod transform;
mod rigidbody;
mod gravity;
mod movement;
mod sprite;
mod aabb;

use aabb::{CollisionSystem, AABB};
use gravity::Gravity;
use movement::{MovementSystem, Movement};
use physics::PhysicsSystem;
use rand::{thread_rng, Rng};
use rigidbody::Rigidbody;
use sprite::{Sprite, RenderSystem};
use system::SystemManager;
use component::ComponentManager;
use ggez::{
    graphics,
    graphics::Color,
    conf,
    GameError,
    GameResult,
    Context,
    ContextBuilder,
    event,
};
use transform::Transform;
use vector2::Vector2;

struct State {
    system_manager: SystemManager,
    component_manager: ComponentManager,
}

impl State {
    fn new(ctx: &Context) -> Self {
        let mut s = Self {
            system_manager: SystemManager::new(),
            component_manager: ComponentManager::new(),
        };

        s.system_manager.register_system::<PhysicsSystem>();
        s.system_manager.register_system::<MovementSystem>();
        s.system_manager.register_system::<RenderSystem>();
        s.system_manager.register_system::<CollisionSystem>();

        s.system_manager.entity_system.create_entity();

        s.component_manager.add_component::<Transform>(0);
        s.component_manager.add_component::<Rigidbody>(0);
        s.component_manager.add_component::<Gravity>(0);
        s.component_manager.add_component::<Movement>(0);
        s.component_manager.add_component::<Sprite>(0);
        s.component_manager.add_component::<AABB>(0);

        s.component_manager.get_components_mut::<Transform>().unwrap().get_entity_component_mut(0).unwrap().pos = Vector2 { x: 64.0, y: 64.0 };
        s.component_manager.get_components_mut::<Sprite>().unwrap().get_entity_component_mut(0).unwrap().color = Color::YELLOW;

        let (screen_w, screen_h) = graphics::drawable_size(ctx);

        for e in 1..15 {
            s.system_manager.entity_system.create_entity();

            let mut rng = thread_rng();
            let x: f32 = rng.gen_range(0.0..screen_w);
            let y: f32 = rng.gen_range(0.0..screen_h);
    
            s.component_manager.add_component::<Transform>(e);
            s.component_manager.add_component::<Sprite>(e);
            s.component_manager.add_component::<AABB>(e);

            s.component_manager.get_components_mut::<Transform>().unwrap().get_entity_component_mut(e).unwrap().set_pos(Vector2 { x, y });
        }

        s
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.system_manager.update(ctx, &mut self.component_manager);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);

        self.system_manager.draw(ctx, &self.component_manager)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let mut c = conf::Conf::new();

    c.window_mode.width = 1920.0;
    c.window_mode.height = 1080.0;
    c.window_setup.title = String::from("ggez test by benji");

    let (ctx, event_loop) = ContextBuilder::new("ggez_test", "benji")
        .default_conf(c)
        .build()
        .unwrap();

    let state = State::new(&ctx);

    event::run(ctx, event_loop, state);
}
