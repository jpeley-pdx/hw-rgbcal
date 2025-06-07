use crate::*;

pub type Adc = saadc::Saadc<'static, 1>;

pub struct Knob(Adc);

impl Knob {

    // Constuctor for Knob
    pub async fn new(adc: Adc) -> Self {
        adc.calibrate().await;
        Self(adc)
    }

    // Function to read the current knob value
    pub async fn measure(&mut self) -> u8 {
        //init buf
        let mut buf = [0];

        // acquire ADC sample
        self.0.sample(&mut buf).await;

        // convert sample
        let raw = buf[0].clamp(0, 0x7fff) as u16;

        // scale sample
        let scaled = raw as f32 / 10_000.0;

        // calculate result
        let result = ((LEVELS + 2) as f32 * scaled - 2.0)
            .clamp(0.0, (LEVELS - 1) as f32)
            .floor();
        result as u8
    }
}
