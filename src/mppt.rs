/// Structure des mesures courantes
pub struct MpptContext {
    pub duty_cycle: f32, // Rapport cyclique actuel (entre 0.0 et 1.0)
    duty_step: f32,      // le pas du duty cycle
    prev_voltage: f32,
    prev_current: f32,
}

impl MpptContext {
    pub fn new() -> Self {
        MpptContext {
            duty_cycle: 0.5,      // initialiser le duty cycle à 50%
            duty_step: 0.01,      // Pas d'incrémentation à 1 %
            prev_voltage: 0.0,
            prev_current: 0.0,
        }
    }

    /// Mise à jour des mesures de tension et courant
    pub fn update_measurements(&mut self, voltage: f32, current: f32) {
        self.prev_voltage = voltage;
        self.prev_current = current;
    }

    /// Exécution de l'algorithme P&O
    pub fn inc_cond_step(&mut self) {
        let dv = self.prev_voltage - self.prev_voltage;
        let di = self.prev_current - self.prev_current;

        if dv.abs() < 1e-5 {
            if di > 0.0 {
                self.duty_cycle += self.duty_step;
            } else if di < 0.0 {
                self.duty_cycle -= self.duty_step;
            }
        } else {
            let inc_cond = di / dv;
            let cond = -self.prev_current / self.prev_voltage;

            if inc_cond > cond {
                self.duty_cycle += self.duty_step;
            } else if inc_cond < cond {
                self.duty_cycle -= self.duty_step;
            }
        }

        // Duty cycle
        self.duty_cycle = self.duty_cycle.clamp(0.0, 1.0);
    }
}
