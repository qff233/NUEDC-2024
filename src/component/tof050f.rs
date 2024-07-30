use defmt::debug;
use embassy_stm32::{mode::Blocking, usart::Uart};

pub struct Tof050f<'a> {
    uart: Uart<'a, Blocking>,
}

impl<'a> Tof050f<'a> {
    pub fn new(mut uart: Uart<'a, Blocking>) -> Self {
        let mut buffer: [u8; 8] = [0x01, 0x06, 0x00, 0x04, 0x00, 0x01, 0x09, 0xCB];
        uart.blocking_write(&buffer).unwrap();
        uart.blocking_flush().unwrap();
        uart.blocking_read(&mut buffer).unwrap();
        // debug!("set_mode recvbuffer: {}", buffer);

        buffer = [0x01, 0x06, 0x00, 0x05, 0x00, 0x00, 0x99, 0xCB];
        uart.blocking_write(&buffer).unwrap();
        uart.blocking_flush().unwrap();
        uart.blocking_read(&mut buffer).unwrap();
        // debug!("set_auto_send recvbuffer: {}", buffer);
        Self { uart }
    }

    pub fn get_value(&mut self) -> f32 {
        let send_buffer: [u8; 8] = [0x01, 0x03, 0x00, 0x10, 0x00, 0x01, 0x85, 0xCF];
        self.uart.blocking_write(&send_buffer).unwrap();
        let mut recv_buffer: [u8; 7] = [0; 7];
        self.uart.blocking_read(&mut recv_buffer).unwrap();
        // debug!("recv buffer: {}", recv_buffer);

        let result: u32 = ((recv_buffer[3] as u32) << 2) | recv_buffer[4] as u32;
        // debug!("tof050f: {}mm", result);
        result as f32 / 10.0
    }
}
