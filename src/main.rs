use rig::completion::Prompt;
use rig::providers::ollama;
use std::error::Error;
mod sim;
use sim::{PIDController, PIDParams, System, calculate_performance_metrics};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ollama_client = ollama::Client::new();
    let ai_tuner = ollama_client.agent("mistral").build();

    let mut system = System::new();
    let mut pid = PIDController::new(1.0, 0.1, 0.05); // Initial parameters
    let setpoint = 1.0;
    let dt = 0.01;
    let simulation_steps = 1000;

    for iteration in 0..10 {
        // Run 10 tuning iterations
        let mut response = Vec::new();

        // Run simulation
        for _ in 0..simulation_steps {
            let control_signal = pid.calculate(setpoint, system.position, dt);
            system.update(control_signal, dt);
            response.push(system.position);
        }

        let (settling_time, max_overshoot, steady_state_error) =
            calculate_performance_metrics(&response, setpoint, dt);

        println!(
            "Iteration {}: ST = {:.2}, MO = {:.2}, SSE = {:.4}",
            iteration, settling_time, max_overshoot, steady_state_error
        );

        // Ask AI to suggest new PID parameters
        let prompt = format!(
            "Current PID parameters: Kp = {:.2}, Ki = {:.2}, Kd = {:.2}\n\
            Performance metrics:\n\
            Settling Time: {:.2}\n\
            Max Overshoot: {:.2}\n\
            Steady State Error: {:.4}\n\
            Suggest new PID parameters to improve performance. \
            Respond ONLY with a JSON object containing 'kp', 'ki', and 'kd' fields, DON'T PROVIDE AN EXPLANATION.",
            pid.kp, pid.ki, pid.kd, settling_time, max_overshoot, steady_state_error
        );

        let ai_response = ai_tuner.prompt(&prompt).await?;

        let new_params: PIDParams = match serde_json::from_str(&ai_response) {
            Ok(new_params) => new_params,
            Err(_) => {
                println!("Failed to Parse JSON: {ai_response}");
                PIDParams::new(1.0, 0.1, 0.05)
            }
        };

        // Update PID parameters
        pid = PIDController::new(new_params.kp, new_params.ki, new_params.kd);

        // Reset system for next iteration
        system = System::new();
    }

    Ok(())
}
