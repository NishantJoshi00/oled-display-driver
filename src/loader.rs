use embedded_graphics::{prelude::*, pixelcolor::BinaryColor};

#[cfg(feature = "sim")]
pub mod disp {
    use std::ops::{Deref, DerefMut};

    use embedded_graphics_simulator::{SimulatorDisplay, OutputSettingsBuilder, Window};

    use super::*;

    pub struct MockDisplay {
        inner: SimulatorDisplay<BinaryColor>,
        window: Window
    }

    impl Deref for MockDisplay {
        type Target = SimulatorDisplay<BinaryColor>;
        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }

    impl MockDisplay {
        pub fn new() -> Self {
            let out_set = OutputSettingsBuilder::new().build();
            MockDisplay {
                inner: SimulatorDisplay::<BinaryColor>::new(Size::new(128, 64)),
                window: Window::new("Simulator", &out_set)
            }
        }
        pub fn flush(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            Ok(self.window.update(&self.inner))
        }

        pub fn clear(&mut self) {
            self.inner.clear(BinaryColor::default()).unwrap()
        }
    }

    impl DerefMut for MockDisplay {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.inner
        }
    }

    pub fn get_display() -> Result<MockDisplay, Box<dyn std::error::Error>> {
        Ok(MockDisplay::new())
    }
}



#[cfg(feature = "board")]
pub mod disp {
    use ssd1306::prelude::*;
    pub fn get_display() -> Result<ssd1306::Ssd1306<ssd1306::I2CInterface<rppal::i2c::I2c>, DisplaySize128x64, ssd1306::mode::BufferedGraphicsMode<DisplaySize128x64>>, Box<dyn std::error::Error>> {

        let i2c_rpi_interface = rppal::i2c::I2c::new()?;
        let interface = ssd1306::I2CDisplayInterface::new(i2c_rpi_interface);
        let mut display = ssd1306::Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate180).into_buffered_graphics_mode();
        display.init()?;
        Ok(display)
    }
}

