use tetra::graphics::{self, Color, Texture, Rectangle};
use tetra::math::Vec2;
use tetra::window;
use tetra::{Context, ContextBuilder, State};
use tetra::input::{self, Key};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const GRAVITY: f32 = 9.8;
const START_POS: Vec2<f32> = Vec2::new(100.0, WINDOW_HEIGHT - 120.0);

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
    fn new(texture: Texture, velocity: Vec2<f32>) -> Entity {
        Entity {
            texture,
            angle : 0.,
            position : START_POS,
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
    fn setOrigin(&mut self, point: Vec2<f32>) {
        self.position.x = point.x - (self.width()/2.0);
        self.position.y = point.y - (self.height()/2.0);
    }
}

struct GameState {
    projectile: Entity,
    start_pos_demo: Entity,
    isPaused : bool,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let javelin_texture = Texture::new(ctx, "./resources/arrow.png")?;
        let demo_ball_texture = Texture::new(ctx, "./resources/ball.png")?;
        
        let velocity = Vec2::new(
            50.0,
            -50.0,
        );

        let mut projectile = Entity::new(javelin_texture, velocity);
        projectile.setOrigin(START_POS);

        Ok(GameState {
            projectile: projectile,
            start_pos_demo: Entity::new(demo_ball_texture, velocity),
            isPaused: false,
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.2, 0.3, 0.9));
        let drawparams = graphics::DrawParams::new()
            .origin(self.projectile.origin())
            .scale(Vec2::new(0.8, 0.6))
            .position(self.projectile.position)
            .rotation(self.projectile.angle);
        graphics::draw(ctx,&self.start_pos_demo.texture, START_POS);
        graphics::draw(ctx,&self.projectile.texture, drawparams);
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        const DESIRED_FPS: u32 = 60;
        let dt = 1.0 / (DESIRED_FPS as f32);
        // update_position
        if !self.isPaused
        {
            self.projectile.position.x += self.projectile.velocity.x * dt;
            let vY = self.projectile.velocity.y;
            self.projectile.position.y += (vY*dt) - (GRAVITY*dt*dt) / 2.0;

            // update velocity.y
            self.projectile.velocity.y += GRAVITY * dt;

            // update angle, in radian
            //let temp = - self.projectile.velocity.x / self.projectile.velocity.y;
            self.projectile.angle = (self.projectile.velocity.y).atan2(self.projectile.velocity.x) + (f32::consts::FRAC_PI_2);
        }

        // Pause with space for a sec.
        self.isPaused = if input::is_key_down(ctx, Key::Space){
            true
        } else {
            false
        };

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