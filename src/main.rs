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
    pub acceleration: vec::MathVec2<physics::Acceleration>,
    pub velocity: vec::MathVec2<physics::Velocity>,
    pub position: vec::MathVec2<physics::Meter>,
    pub time: physics::Second,
    pub angle: f64, // radians
}

impl Rocket {
    pub fn new(bottle: Bottle, mass: physics::Kilogram, acceleration: vec::MathVec2<physics::Acceleration>, velocity: vec::MathVec2<physics::Velocity>, position: vec::MathVec2<physics::Meter>, time: physics::Second, angle: f64) -> Self {
        Rocket {
            bottle,
            mass,
            acceleration,
            velocity,
            position,
            time,
            angle,
        }
    }

    pub fn update_state(&mut self, dt: physics::Second) {
        const k: f64 = 0.1; // air resistance coefficient
        // calculate force
        let R = physics::Newton(k * self.velocity.magnitude().0); // resistance
        let F = self.bottle.thrust - R; // force
        let Fx = F * self.angle.cos();
        let Fy = F * self.angle.sin();

        // calculate acceleration
        let ax = Fx / self.mass;
        let ay = Fy / self.mass;

        // update acceleration and velocity
        self.acceleration = vec::MathVec2::new(ax, ay);
        self.velocity = vec::MathVec2::new(self.velocity.x + ax * dt.0, self.velocity.y + ay * dt.0);

        // update position
        self.position = vec::MathVec2::new(self.position.x + self.velocity.x * dt.0, self.position.y + self.velocity.y * dt.0);

        // update time
        self.time = physics::Second(self.time.0 + dt.0);

        // update bottle state
        self.bottle.update_state(dt);
    }
    
}


fn main() {
    let bottle = Bottle::new(physics::Kilogram::new(1.0), physics::Newton::new(100.0), physics::Newton::new(0.0), physics::SquareMeter::new(0.01));
    let mass = physics::Kilogram::new(1.0);
    let acceleration = vec::MathVec2::new(physics::Acceleration::new(0.0), physics::Acceleration::new(0.0));
    let velocity = vec::MathVec2::new(physics::Velocity::new(0.0), physics::Velocity::new(0.0));
    let position = vec::MathVec2::new(physics::Meter::new(0.0), physics::Meter::new(0.0));
    let time = physics::Second::new(0.0);
    let angle = 0.0;

    let mut rocket = Rocket::new(bottle, mass, acceleration, velocity, position, time, angle);

    let dt = physics::Second::new(0.1);
    loop {
        rocket.update_state(dt);
        println!("{:?}", rocket);
        if rocket.position.y.0 < 0.0 {
            break;
        }
    }
}