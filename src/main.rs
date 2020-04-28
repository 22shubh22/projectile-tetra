use tetra::graphics::{self, Color, Texture, Rectangle};
use tetra::math::Vec2;
use tetra::window;
use tetra::{Context, ContextBuilder, State};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;

fn main() -> tetra::Result {
    ContextBuilder::new("Javelin", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
    .quit_on_escape(true)
    .build()?
    .run(GameState::new)
}

struct Entity {
    texture: Texture,
    angle: f32,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
}

impl Entity {
    fn new(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Entity {
        Entity {
            texture,
            angle : 0.,
            position,
            velocity,
        }
    }

    fn width(&self) -> f32 {
        self.texture.width() as f32
    }

    fn height(&self) -> f32 {
        self.texture.height() as f32
    }

    fn origin(&self) -> Vec2<f32> {
        Vec2::new (
            self.width() / 2.0 as f32,
            self.height() / 2.0 as f32,
        )
    }
}

struct GameState {
    projectile: Entity,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let javelin_texture = Texture::new(ctx, "./resources/arrow.png")?;
        let position = Vec2::new(
            16.0,
            (WINDOW_HEIGHT - javelin_texture.height() as f32 - 100.0),
        );

        let velocity = Vec2::new(
            20.0,
            20.0,
        );

        Ok(GameState {
            projectile: Entity::new(javelin_texture, position, velocity),
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.2, 0.3, 0.9));

        graphics::draw(ctx,&self.projectile.texture, self.projectile.position);
        Ok(())
    }
}