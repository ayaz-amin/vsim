use simulator::Simulator;

pub mod vehicle;
pub mod simulator;


#[macroquad::main("vsim")]
async fn main() {
    let car = vehicle::Vehicle::new(
        vehicle::ElectricMotor::new(
            2.7, 285.0, 700.0
        ),
        vehicle::VehicleParams::new(
            1500.0, 0.3, 2.6, 0.01, 0.33, vec![2.0, 1.0], 7.898
        )
    );
    let game = Simulator::new(car);
    game.run().await;
}