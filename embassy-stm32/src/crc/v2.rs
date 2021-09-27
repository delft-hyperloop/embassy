use crate::pac::CRC as PAC_CRC;
use crate::pac::crc::vals;
use crate::peripherals::CRC;
use crate::rcc::sealed::RccPeripheral;

pub struct Crc {
    _peripheral: CRC,
    _config: CrcConfig,
}

pub enum CrcConfigError {
    InvalidPolynomial
}

pub struct CrcConfig {
    reverse_in: CrcInputReverseConfig,
    reverse_out: bool,
    poly_size: PolySize,
    crc_init_value: u32,
    crc_poly: u32,
}

pub enum CrcInputReverseConfig {
    None,
    Byte,
    Halfword,
    Word,
}

impl CrcConfig {
    pub fn new(reverse_in: CrcInputReverseConfig, reverse_out: bool, poly_size: PolySize, crc_init_value: u32, crc_poly: u32) -> Result<Self, CrcConfigError> {
        // As Per RM0091 (DocID018940 Rev 9), Even polynomials are not supported.
        if crc_poly % 2 == 0 {
            Err(CrcConfigError::InvalidPolynomial)
        } else {
            Ok(CrcConfig { reverse_in, reverse_out, poly_size, crc_init_value, crc_poly })
        }
    }
}

pub enum PolySize {
    Width7,
    Width8,
    Width16,
    Width32,
}

impl Crc {
    /// Instantiates the CRC32 peripheral and initializes it to default values.
    pub fn new(peripheral: CRC, init_value: u32, config: CrcConfig) -> Self {
        // Note: enable and reset come from RccPeripheral.
        // enable CRC clock in RCC.
        CRC::enable();
        // Reset CRC to default values.
        CRC::reset();
        let mut instance = Self {
            _peripheral: peripheral,
            _config: config,
        };
        unimplemented!();
        // instance.init();
        // instance
    }



    pub fn reset(&mut self) {
        unsafe { PAC_CRC.cr().modify(|w| w.set_reset(true)); }
    }


    fn reconfigure(&mut self) {
        unsafe {
            // Init CRC value
            PAC_CRC.init().write_value(self._config.crc_init_value);

            PAC_CRC.cr().modify(|w| {
                // configure reverse output
                w.set_rev_out(
                    match self._config.reverse_out {
                        true => { vals::RevOut::REVERSED }
                        false => { vals::RevOut::NORMAL }
                    }
                );
                // configure reverse input
                w.set_rev_in(
                    match self._config.reverse_in {
                        CrcInputReverseConfig::None => { vals::RevIn::NORMAL }
                        CrcInputReverseConfig::Byte => { vals::RevIn::BYTE }
                        CrcInputReverseConfig::Halfword => { vals::RevIn::HALFWORD }
                        CrcInputReverseConfig::Word => { vals::RevIn::WORD }
                    }
                );
                // configure the polynomial.
                w.set_polysize(
                    match self._config.poly_size {
                        PolySize::Width7 => { vals::Polysize::POLYSIZE7 }
                        PolySize::Width8 => { vals::Polysize::POLYSIZE8 }
                        PolySize::Width16 => { vals::Polysize::POLYSIZE16 }
                        PolySize::Width32 => { vals::Polysize::POLYSIZE32 }
                    }
                )

            })
        }

        self.reset();
    }
}