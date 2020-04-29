use tetra::graphics::{self, Color, Texture, Rectangle};
use tetra::math::Vec2;
use tetra::window;
use tetra::{Context, ContextBuilder, State};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const GRAVITY: f32 = 9.8;

use std::f32;

fn main() -> tetra::Result {
    ContextBuilder::new("Javelin", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
    .quit_on_escape(true)
    .build()?
    .run(GameState::new)
}

struct Entity {
    texture: Texture,
    angle: f32,         // angle in radian [0,2*pi] //std::f32::consts::PI
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
            32.0,
            (WINDOW_HEIGHT - javelin_texture.height() as f32 - 100.0),
        );

        let velocity = Vec2::new(
            50.0,
            -50.0,
        );

        Ok(GameState {
            projectile: Entity::new(javelin_texture, position, velocity),
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.2, 0.3, 0.9));
        let drawparams = graphics::DrawParams::new()
            .origin(self.projectile.origin())
            .position(self.projectile.position)
            .rotation(self.projectile.angle);
        graphics::draw(ctx,&self.projectile.texture, drawparams);
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        const DESIRED_FPS: u32 = 60;
        let dt = 1.0 / (DESIRED_FPS as f32);
        // update_position
        self.projectile.position.x += self.projectile.velocity.x * dt;
        let vY = self.projectile.velocity.y;
        self.projectile.position.y += (vY*dt) - (GRAVITY*dt*dt) / 2.0;

        // update velocity.y
        self.projectile.velocity.y += GRAVITY * dt;

        // update angle, in radian
        //let temp = - self.projectile.velocity.x / self.projectile.velocity.y;
        self.projectile.angle = (self.projectile.velocity.y).atan2(self.projectile.velocity.x) + (f32::consts::FRAC_PI_2);

        Ok(())
    }
}

/*fn update_position(&mut self, dt: f32)
{
    self.projectile.position.x = positionX(dt);
    self.projectile.position.y = positionY(dt);
}

fn update_positionX(&mut self, dt: f32) -> f32
{
    self.projectile.velocity.x * dt
}

fn update_positionY(&mut self, dt: f32) -> f32
{
    let vY = self.projectile.velocity.y;
    (vY*dt) - (10.0*dt*dt) / 2.0 
}*/