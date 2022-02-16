mod vector2;
mod entity;
mod system;
mod component;
mod physics;
mod transform;
mod rigidbody;
mod gravity;
mod movement;

use gravity::Gravity;
use movement::{MovementSystem, Movement};
use physics::PhysicsSystem;
use rigidbody::Rigidbody;
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
    event, mint::Vector2,
};
use transform::Transform;

struct State {
    system_manager: SystemManager,
    component_manager: ComponentManager,
}

impl State {
    fn new() -> Self {
        let mut s = Self {
            system_manager: SystemManager::new(),
            component_manager: ComponentManager::new(),
        };

        s.system_manager.register_system::<PhysicsSystem>();
        s.system_manager.register_system::<MovementSystem>();

        s.system_manager.entity_system.create_entity();

        let transform = Transform::default();
        let rigidbody = Rigidbody::default();
        let gravity = Gravity(0.0);
        let movement = Movement { speed: 256.0 };

        s.component_manager.add_component::<Transform>(0, transform);
        s.component_manager.add_component::<Rigidbody>(0, rigidbody);
        s.component_manager.add_component::<Gravity>(0, gravity);
        s.component_manager.add_component::<Movement>(0, movement);

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

        let rect = graphics::Rect::new(0.0, 0.0, 32.0, 32.0);
        let rect_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, Color::WHITE)?;

        let player_position = self.component_manager.get_components::<Transform>().unwrap().get_entity_component(0).unwrap().pos;
        let pos = Vector2 {
            x: player_position.x,
            y: player_position.y
        };
        let draw_params = graphics::DrawParam::new()
            .dest(pos);

        graphics::draw(ctx, &rect_mesh, draw_params)?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let state = State::new();

    let mut c = conf::Conf::new();

    c.window_mode.width = 1920.0;
    c.window_mode.height = 1080.0;
    c.window_setup.title = String::from("ggez test by benji");

    let (ctx, event_loop) = ContextBuilder::new("ggez_test", "benji")
        .default_conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);
}
