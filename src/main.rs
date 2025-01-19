mod mppt;
mod adc;
mod pwm;
use crate::mppt::MpptContext;
use crate::adc::read_voltage_current;
use crate::pwm::set_duty_cycle;


fn main() {
    // Initialisation de l'algorithme MPPT
    let mut mppt = MpptContext::new();

    loop {
        // Lire les valeurs de tension et de courant depuis l'ADC
        let (voltage, current) = read_voltage_current();

        // Mise à jour des mesures
        mppt.update_measurements(voltage, current);
        mppt.inc_cond_step();

        //  PWM
        set_duty_cycle(mppt.duty_cycle);

        // Pause pour simuler le temps réel
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
