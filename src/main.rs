mod vehicle;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Text};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::{KeyCode, KeyInput};

struct Simulator<T: vehicle::Motor> {
    car: vehicle::Vehicle<T>
}

impl<T: vehicle::Motor> Simulator<T> {
    fn new(car: vehicle::Vehicle<T>) -> Simulator<T> {
        Simulator {car}
    }
}

impl<T: vehicle::Motor> EventHandler for Simulator<T> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.car.update(0.02);
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLUE);
        
        let fps = Text::new(format!("FPS: {}", ctx.time.fps()));
        let speed = Text::new(format!("Velocity: {}", self.car.velocity * 3.6));
        let rpm = Text::new(format!("RPM: {}", self.car.rpm));
        
        canvas.draw(&fps, graphics::DrawParam::from([200.0, 0.0]).color(Color::WHITE));
        canvas.draw(&speed, graphics::DrawParam::from([200.0, 50.0]).color(Color::WHITE));
        canvas.draw(&rpm, graphics::DrawParam::from([200.0, 100.0]).color(Color::WHITE));
        
        canvas.finish(ctx)
    }
    
    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, repeat: bool) -> GameResult {
        match input.keycode {
            Some(KeyCode::Up) => {
                self.car.throttle(true);
            },
            Some(KeyCode::Q) => {
                self.car.shift_up();
            },
            Some(KeyCode::E) => {
                self.car.shift_down();
            },
            _ => {}
        }
        Ok(())
    }
    
    fn key_up_event(&mut self, ctx: &mut Context, input: KeyInput) -> GameResult {
        match input.keycode {
            Some(KeyCode::Up) => {
                self.car.throttle(false);
            },
            _ => {}
        }
        Ok(())
    }
}

fn main() {
    let car = vehicle::Vehicle::new(
        vehicle::ElectricMotor::new(
            2.7, 285.0, 700.0
        ),
        vehicle::VehicleParams::new(
            1500.0, 0.3, 2.6, 0.01, 0.33, vec![2.0, 1.0], 7.898
        )
    );
    
    let (mut ctx, event_loop) = ContextBuilder::new("Game", "Me").build().expect("Failed");
    let game = Simulator::new(car);
    event::run(ctx, event_loop, game);
}