/// Convertit une valeur brute d'ADC en volts
pub fn adc_to_voltage(adc_value: u16, vref: f32) -> f32 {
    (adc_value as f32 * vref) / 1023.0
}
