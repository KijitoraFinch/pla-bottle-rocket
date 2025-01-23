mod physics;
mod vec;
#[derive(Debug)]
pub struct Bottle {
    pub water: physics::Kilogram,
    pub pressure: physics::Newton,
    pub thrust: physics::Newton,
    nozzle_area: physics::SquareMeter,

}

impl Bottle {
    pub fn new(water: physics::Kilogram, pressure: physics::Newton, thrust: physics::Newton, nozzle_area: physics::SquareMeter) -> Self {
        Bottle {
            water,
            pressure,
            thrust,
            nozzle_area,
        }
    }

    pub fn update_state(&mut self, dt: physics::Second) {
        // update water mass, pressure, and thrust using bernoulli's equation

        // update water mass
        self.water = physics::Kilogram(self.water.0 - (self.thrust.0 / physics::GRAVITY.0) * dt.0);

        // update pressure
        self.pressure = physics::Newton(self.pressure.0 - (self.thrust.0 / self.nozzle_area.0) * dt.0);

        // update thrust
        self.thrust = physics::Newton(self.pressure.0 * self.nozzle_area.0);
    }
}


#[derive(Debug)]
pub struct Rocket {
    pub bottle: Bottle,
    pub mass: physics::Kilogram,
    pub acceleration: vec::MathVec3<physics::Acceleration>,
    pub velocity: vec::MathVec3<physics::Velocity>,
    pub position: vec::MathVec3<physics::Meter>,
    pub time: physics::Second,
}

impl Rocket {
    pub fn new(bottle: Bottle, mass: physics::Kilogram, acceleration: vec::MathVec3<physics::Acceleration>, velocity: vec::MathVec3<physics::Velocity>, position: vec::MathVec3<physics::Meter>, time: physics::Second) -> Self {
        Rocket {
            bottle,
            mass,
            acceleration,
            velocity,
            position,
            time,
        }
    }

    pub fn update_state(&mut self, dt: physics::Second) {
        // calculate force

    }
    
}
