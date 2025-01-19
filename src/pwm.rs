/// Simule le réglage du duty cycle à la place des registres PWM du hardware
pub fn set_duty_cycle(duty: f32) {
    println!("Duty cycle réglé à : {:.2}%", duty * 100.0);
}
