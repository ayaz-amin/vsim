use std::f32::consts::PI;

const AIR_DENSITY: f32 = 1.29;
const TAU_DIV_60_INV: f32 = 60.0 / (2.0 * PI);
const TAU_DIV_60: f32 = 1.0 / TAU_DIV_60_INV;

pub trait Motor {
    fn update_torque(&mut self, rpm: f32) -> f32;
}

pub struct ElectricMotor {
    motor_const: f32,
    peak_power: f32,
    peak_torque: f32,
    base_speed: f32
}

impl ElectricMotor {
    pub fn new(
        motor_const: f32,
        supply_current: f32,
        voltage: f32
    ) -> Self {
        let peak_power: f32 = supply_current * voltage * 0.9;
        let peak_torque: f32 = supply_current * motor_const;
        let base_speed: f32 = (peak_power * 30.0)/(peak_torque * PI);
        
        Self {
            motor_const,
            peak_power,
            peak_torque,
            base_speed
        }
    }
}

impl Motor for ElectricMotor {
    fn update_torque(&mut self, rpm: f32) -> f32 {
        if rpm <= self.base_speed {
            self.peak_torque
        } 
        else {
            (self.peak_power * 30.0)/(rpm * PI)
        }
    }
}

pub struct VehicleParams {
    mass: f32,
    drag_coefficient: f32,
    rolling_resistance_coefficient: f32,
    wheel_radius: f32,
    transmission: Vec<f32>,
    differential: f32
}

impl VehicleParams {
    pub fn new(
        mass: f32,
        friction_coefficient: f32,
        frontal_area: f32,
        rolling_resistance_coefficient: f32,
        wheel_radius: f32,
        transmission: Vec<f32>,
        differential: f32
    ) -> Self {
        let drag_coefficient: f32 = 
            0.5 * friction_coefficient * frontal_area * AIR_DENSITY;
        
        Self {
            mass, drag_coefficient,
            rolling_resistance_coefficient,
            wheel_radius, transmission,
            differential
        }
    }
}

pub struct Vehicle<T: Motor> {
    motor: T,
    params: VehicleParams,
    gear_num: usize,
    press_throttle: bool,
    pub rpm: f32,
    pub velocity: f32
}

impl<T: Motor> Vehicle<T> {
    pub fn new(motor: T, params: VehicleParams) -> Self {
        Self {
            motor, params,
            gear_num: 0, press_throttle: false, rpm: 0.0,
            velocity: 0.0
        }
    }
    
    pub fn update(&mut self, delta_time: f32) {
        let torque: f32 = 
            if self.press_throttle {self.motor.update_torque(self.rpm)} else {0.0};
                
        let traction: f32 = 
            (torque * self.params.transmission[self.gear_num] * self.params.differential) 
                / self.params.wheel_radius;
        
        let drag: f32 = -self.params.drag_coefficient * self.velocity * self.velocity.abs();
        
        let rolling_resistance: f32 = 
            -self.params.rolling_resistance_coefficient * self.params.mass * 9.81;
        
        let longitudinal: f32 = traction + drag + rolling_resistance;
        
        let acceleration: f32 = longitudinal / self.params.mass;
        
        self.velocity += acceleration * delta_time;
        
        self.velocity = self.velocity.max(0.0);
        
        self.rpm = 
            (self.velocity / self.params.wheel_radius) 
                * self.params.transmission[self.gear_num] 
                * self.params.differential * TAU_DIV_60_INV;
    }
    
    pub fn throttle(&mut self, press_throttle: bool) {
        self.press_throttle = press_throttle;
    }
    
    pub fn shift_up(&mut self) {
        if self.gear_num < self.params.transmission.len() - 1 {
            self.gear_num += 1;
        }
    }
    
    pub fn shift_down(&mut self) {
        if self.gear_num > 0 {
            self.gear_num -= 1;
        }
    }
    
    pub fn reset(&mut self) {
        self.gear_num = 0;
        self.rpm = 0.0;
        self.velocity = 0.0;
    }
}