use std::{
    error::Error,
    sync::{
        atomic::{AtomicU16, Ordering},
        Arc,
    },
};

use esp_idf_svc::hal::{
    adc::{AdcContConfig, AdcContDriver, AdcMeasurement, Attenuated, ADC1},
    delay,
    gpio::Gpio32,
    i2s::I2S0,
};
use log::{error, warn};

#[derive(Clone)]
pub struct PotValue {
    latest: Arc<AtomicU16>,
}

impl PotValue {
    pub fn latest(&self) -> u16 {
        self.latest.load(Ordering::Relaxed)
    }
}

pub fn spawn(
    adc1: ADC1<'static>,
    i2s0: I2S0<'static>,
    adc_pin: Gpio32<'static>,
) -> Result<PotValue, Box<dyn Error>> {
    let latest = Arc::new(AtomicU16::new(0));
    let task_latest = latest.clone();

    std::thread::Builder::new()
        .name("adc-read-poti".into())
        .stack_size(4096)
        .spawn(move || {
            let adc_channel = Attenuated::db12(adc_pin);
            let adc_config = AdcContConfig::default();
            let mut adc = match AdcContDriver::new(adc1, i2s0, &adc_config, adc_channel) {
                Ok(adc) => adc,
                Err(err) => {
                    error!("ADC init failed: {err}");
                    return;
                }
            };

            if let Err(err) = adc.start() {
                error!("Failed to start ADC: {err}");
                return;
            }

            let mut samples = [AdcMeasurement::default(); 16];

            loop {
                match adc.read(&mut samples, delay::BLOCK) {
                    Ok(count) if count > 0 => {
                        let sum: u16 = samples[..count].iter().map(|sample| sample.data()).sum();
                        let avg = sum / count as u16;

                        task_latest.store(avg, Ordering::Relaxed);
                    }
                    Ok(_) => {}
                    Err(err) => {
                        warn!("Failed to read ADC: {err}");
                    }
                }
            }
        })?;

    Ok(PotValue { latest })
}
