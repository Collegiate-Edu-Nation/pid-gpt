use serde::{Deserialize, Serialize};

// Simulate a second-order system
pub struct System {
    pub position: f64,
    pub velocity: f64,
}

impl System {
    pub fn new() -> Self {
        System {
            position: 0.0,
            velocity: 0.0,
        }
    }

    pub fn update(&mut self, force: f64, dt: f64) {
        let acceleration = force - 0.1 * self.velocity - 2.0 * self.position;
        self.velocity += acceleration * dt;
        self.position += self.velocity * dt;
    }
}

// PID Controller
pub struct PIDController {
    pub kp: f64,
    pub ki: f64,
    pub kd: f64,
    integral: f64,
    prev_error: f64,
}

impl PIDController {
    pub fn new(kp: f64, ki: f64, kd: f64) -> Self {
        PIDController {
            kp,
            ki,
            kd,
            integral: 0.0,
            prev_error: 0.0,
        }
    }

    pub fn calculate(&mut self, setpoint: f64, current_value: f64, dt: f64) -> f64 {
        let error = setpoint - current_value;
        self.integral += error * dt;
        let derivative = (error - self.prev_error) / dt;
        let output = self.kp * error + self.ki * self.integral + self.kd * derivative;
        self.prev_error = error;
        output
    }
}

// Performance metrics
pub fn calculate_performance_metrics(response: &[f64], setpoint: f64, dt: f64) -> (f64, f64, f64) {
    let steady_state_error = (response.last().unwrap() - setpoint).abs();

    let mut max_overshoot = 0.0;
    for &value in response.iter() {
        let overshoot = (value - setpoint).abs();
        if overshoot > max_overshoot {
            max_overshoot = overshoot;
        }
    }

    let settling_time = response.len() as f64 * dt; // Simplified

    (settling_time, max_overshoot, steady_state_error)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PIDParams {
    pub kp: f64,
    pub ki: f64,
    pub kd: f64,
}

impl PIDParams {
    pub fn new(kp: f64, ki: f64, kd: f64) -> PIDParams {
        PIDParams { kp, ki, kd }
    }
}
