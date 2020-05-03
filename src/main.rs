use tetra::graphics::{self, Color, Texture, Rectangle};
use tetra::math::Vec2;
use tetra::window;
use tetra::{Context, ContextBuilder, State};
use tetra::input::{self, Key, MouseButton};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const GRAVITY: f32 = 9.8;
const START_POS: Vec2<f32> = Vec2::new(100.0, WINDOW_HEIGHT - 120.0);
const ARROw_VEL_MAG: f32 = 100.0;

use std::f32;

fn main() -> tetra::Result {
    ContextBuilder::new("Javelin", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
    .quit_on_escape(true)
    .build()?
    .run(GameState::new)
}

struct UtilityGraphics {
    texture: Texture,
    position: Vec2<f32>,
}

impl UtilityGraphics {
    fn new(texture: Texture, position: Vec2<f32>) -> UtilityGraphics {
        UtilityGraphics {
            texture,
            position,
        }
    }
}

struct Entity {
    texture: Texture,
    angle: f32,         // angle in radian [0,2*pi] //std::f32::consts::PI
    position: Vec2<f32>,
    velocity: Vec2<f32>,
}

impl Entity {
    fn new(texture: Texture) -> Entity {
        Entity {
            texture,
            angle : 0.,
            position : START_POS,
            velocity : Vec2::new(0.,0.),
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
        // DEBUG
        println!("setOrigin-position {:?}", self.position);
    }
}

struct GameState {
    projectile: Entity,
    startPos: Entity,
    mouse_ptr: UtilityGraphics,
    isPaused : bool,
    is_arrow_released: bool,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let javelin_texture = Texture::new(ctx, "./resources/arrow.png")?;
        let cross_texture = Texture::new(ctx, "./resources/cross.png")?;

        let mut projectile = Entity::new(javelin_texture);
        projectile.setOrigin(START_POS);
        
        let mut cross = Entity::new(cross_texture);
        cross.setOrigin(START_POS);

        // println!("Projectile origin {:?}", projectile.position + projectile.origin());
        // println!("cross origin {:?}", cross.position + cross.origin());

        // Utility graphics
        let mouse_ptr_texture = Texture::new(ctx, "./resources/ball.png")?;
        let mouse_ptr_position = input::get_mouse_position(ctx).round();
        let mouse_ptr = UtilityGraphics::new(mouse_ptr_texture, mouse_ptr_position);

        Ok(GameState {
            projectile: projectile,
            startPos: cross,
            mouse_ptr,
            isPaused: false,
            is_arrow_released: false,
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // draw screen
        graphics::clear(ctx, Color::rgb(0.2, 0.3, 0.9));
        // draw projectile
        let drawparams_projectile = graphics::DrawParams::new()
            .origin(self.projectile.origin())
            .scale(Vec2::new(0.8, 0.6))
            .position(START_POS)
            .rotation(self.projectile.angle);
        graphics::draw(ctx, &self.projectile.texture, drawparams_projectile);
        // draw origin OR cross
        let drawparams_cross = graphics::DrawParams::new()
            .origin(self.startPos.origin())
            .position(START_POS);
        graphics::draw(ctx, &self.startPos.texture, drawparams_cross);

        // draw mouse_ptr
        graphics::draw(ctx, &self.mouse_ptr.texture, self.mouse_ptr.position);
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        const DESIRED_FPS: u32 = 60;
        let dt = 1.0 / (DESIRED_FPS as f32);
        // update_position
        if self.is_arrow_released
        {
            /*self.projectile.position.x += self.projectile.velocity.x * dt;
            let vY = self.projectile.velocity.y;
            self.projectile.position.y += (vY*dt) - (GRAVITY*dt*dt) / 2.0;

            // update velocity.y
            self.projectile.velocity.y += GRAVITY * dt;

            // update angle, in radian
            //let temp = - self.projectile.velocity.x / self.projectile.velocity.y;
            self.projectile.angle = (self.projectile.velocity.y).atan2(self.projectile.velocity.x) + (f32::consts::FRAC_PI_2);*/
        }

        // Pause with space for a sec.
        self.isPaused = if input::is_key_down(ctx, Key::Space){
            true
        } else {
            false
        };

        //mouse angle set
        /*
            If arrow not released ( bool is_arrow_released ) then set the angle to which the arrow should complete the projectile.
        */
        self.mouse_ptr.position = input::get_mouse_position(ctx).round();

        if !self.is_arrow_released {
            if input::is_mouse_button_down(ctx, MouseButton::Left) {
                let mouse_position = self.mouse_ptr.position;
                // change angle at left part of screen..
                if(START_POS.x > mouse_position.x)
                {
                    self.projectile.angle = ((START_POS.y - mouse_position.y)/(START_POS.x - mouse_position.x)).atan();
                }
            }
            if input::is_mouse_button_released(ctx, MouseButton::Left) {
                self.is_arrow_released = true;
            }
        }

        // quit after a projectile, when arrow origin reach ground

        if self.projectile.position.y > START_POS.y {
            window::quit(ctx);
            println!("Arrow completes projectile!");
        }

        Ok(())
    }
}