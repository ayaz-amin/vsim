use std::{thread::sleep, time::Duration};

use macroquad::prelude::*;

use crate::vehicle;


pub struct Simulator<T: vehicle::Motor> {
    car: vehicle::Vehicle<T>
}

impl<T: vehicle::Motor> Simulator<T> {
    pub fn new(car: vehicle::Vehicle<T>) -> Simulator<T> {
        Simulator {car}
    }
}

impl<T: vehicle::Motor> Simulator<T>{
    pub async fn run(mut self){
        loop {
            clear_background(BLACK);
            self.update();
            self.draw();
            next_frame().await
        }
    }

    fn update(&mut self) {
        self.handle_key_down();
        self.handle_key_up();
        self.car.update(0.02);
    }
    
    fn draw(&self) {
        clear_background(BLUE);

        const OFFSET_X: f32 = 250.0;
        const OFFSET_Y: f32 = 50.0;
        const FONT_SIZE: f32 = 50.0;
        
        draw_text(&format!("fps: {}", get_fps()), OFFSET_X, OFFSET_Y, FONT_SIZE, WHITE);
        draw_text(&format!("velocity: {}", self.car.velocity * 3.6), OFFSET_X, OFFSET_Y + 50.0, FONT_SIZE, WHITE);
        draw_text(&format!("rpm: {}", self.car.rpm), OFFSET_X, OFFSET_Y + 100.0, FONT_SIZE, WHITE);
    }

    fn handle_key_down(&mut self) {
        if is_key_down(KeyCode::Up){
            self.car.throttle(true)
        }else if is_key_down(KeyCode::Q){
                self.car.shift_up();
        }else if is_key_down(KeyCode::E){
            self.car.shift_down();
        }
    }

    fn handle_key_up(&mut self) {
        if is_key_released(KeyCode::Up){
            self.car.throttle(false)
        }
    }
}
