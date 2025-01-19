/// Simule la lecture de la tension et du courant via l'ADC ( absence du convertisseur au moment du portage du projet )
pub fn read_voltage_current() -> (f32, f32) {
    // les valeurs simulé :
    let voltage = 12.0; // Exemple de tension en volts
    let current = 1.5;  // Exemple de courant en ampères
    (voltage, current)
}
