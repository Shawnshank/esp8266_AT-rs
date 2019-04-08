//! ESP8266 device crate

#![no_std]
#![allow(warnings)]

extern crate embedded_hal as hal;
extern crate heapless;
extern crate nb;

use core::fmt::Write;
use hal::blocking::delay;
//use hal::blocking::delay::DelayMs;
use hal::serial;
use heapless::consts::*;
use heapless::String;
use nb::block;

/// Module for AT commands.
/// Referenced by [Espressif AT send_ set](https://www.espressif.com/sites/default/files/documentation/4a-esp8266_at_instruction_set_en.pdf)
pub mod commands;

pub struct esp8266<TX, RX, DELAY> {
    tx: TX,
    rx: RX,
    delay: DELAY,
    received: [u8; 32], // TODO: Max return length from ESP
    connection_status: bool,
    got_ip: bool,
    ip: (u8, u8, u8, u8),
}

impl<TX, RX, DELAY, E> esp8266<TX, RX, DELAY>
where
    TX: serial::Write<u8, Error = E>,
    RX: serial::Read<u8, Error = E>,
    DELAY: delay::DelayMs<u16>,
{
    /// Creates a new ESP8266
    /// # Example STM32F411
    /// ```
    /// #![no_std]
    /// #![no_main]
    ///
    /// extern crate ESP8266;
    /// pub extern crate stm32f4xx_hal as hal;
    ///
    /// use hal::delay::Delay;
    /// use hal::serial::{config::Config, Serial};
    /// use cortex_m_rt::entry;
    ///
    /// #[entry]
    /// fn main() -> ! {
    ///     let dp = stm32::Peripherals::take().unwrap();
    ///     let rcc = dp.RCC.constrain();
    ///     let gpioa = dp.GPIOA.split();
    ///     let mut delay = Delay::new(cp.SYST, clocks);
    ///
    ///     let tx1 = gpioa.pa2.into_alternate_af7();
    ///     let rx1 = gpioa.pa3.into_alternate_af7();
    ///     let esp_config = Config::default().baudrate(115200.bps());
    ///     let esp_serial = Serial::usart2(dp.USART2, (tx2, rx2), esp_config, clocks).unwrap();
    ///
    ///     let (tx, rx) = esp_serial.split();
    ///     let mut esp = ESP8266::esp8266::new(usart_tx, usart_rx, delay).unwrap();
    ///     
    ///     loop {}
    /// }
    /// ```
    pub fn new(tx: TX, rx: RX, delay: DELAY) -> Result<Self, E> {
        let esp8266 = esp8266 {
            tx: tx,
            rx: rx,
            delay: delay,
            received: [0u8; 32], // TODO: Max return length from ESP
            connection_status: false,
            got_ip: false,
            ip: (0, 0, 0, 0),
        };
        Ok(esp8266)
    }

    /// Initializing the connection to a connected ESP device by
    /// checking if there is a device present and turn off AT send_ echoing
    pub fn init(&mut self) -> Result<(), ()> {
        // Switch echoing off
        match self.send(commands::AT_commands::ATE(false)) {
            Ok(_) => {
                self.connection_status = true;
            }
            Err(_) => self.connection_status = false,
        }

        match self.send(commands::AT_commands::AT) {
            Ok(_) => {
                self.connection_status = true;
            }
            Err(_) => self.connection_status = false,
        }

        // TODO: SHOULD PROBABLY RESET THE DEVICE

        // Return
        if !self.connection_status {
            Err(())
        } else {
            Ok(())
        }
    }
    /// Join an access point with given SSID and password
    /// # Example
    /// ```
    /// let ssid = "your_ssid";
    /// let pwd = "your_password";
    /// esp.join_AP(ssid, pwd).unwrap();
    /// ```
    pub fn join_AP(&mut self, ssid: &str, password: &str) -> Result<(), ()> {
        match self.send(commands::AT_commands::CWJAP(ssid, password)) {
            Ok(_) => {
                self.connection_status = true;
            }
            Err(_) => self.connection_status = false,
        }
        // Get the IP of the module
        match self.send(commands::AT_commands::CIFSR) {
            Ok(_) => {
                self.connection_status = true;
            }
            Err(_) => self.connection_status = false,
        }
        // Return
        if !self.connection_status {
            Err(())
        } else {
            Ok(())
        }
    }

    pub fn get_IP(&mut self) -> Result<(), ()> {
        match self.send(commands::AT_commands::CIFSR) {
            Ok(_) => {
                self.connection_status = true;
            }
            Err(_) => self.connection_status = false,
        }
        // Return
        if !self.connection_status {
            Err(())
        } else {
            Ok(())
        }
    }

    /// Creates a TCP server for multiple connections
    pub fn tcp_server(&mut self, port: u16) -> Result<(), ()> {
        match self.send(commands::AT_commands::CWMODE(1)) {
            Ok(_) => {
                self.connection_status = true;
            }
            Err(_) => self.connection_status = false,
        }

        match self.send(commands::AT_commands::CIPMUX(1)) {
            Ok(_) => {
                self.connection_status = true;
            }
            Err(_) => self.connection_status = false,
        }

        match self.send(commands::AT_commands::CIPSERVER_EXT(1, port)) {
            Ok(_) => {
                self.connection_status = true;
            }
            Err(_) => self.connection_status = false,
        }
        // Return
        if !self.connection_status {
            Err(())
        } else {
            Ok(())
        }
    }

    /// Creates a UDP server that listens on all incomming addresses
    pub fn udp_server(&mut self, port: u16) -> Result<(), ()> {
        /* match self.send(commands::AT_commands::CIPSERVER(0)) {
            Ok(_) => {
                self.connection_status = true;
            }
            Err(_) => self.connection_status = false,
        }
        
        match self.send(commands::AT_commands::RST) {
            Ok(_) => {
                self.connection_status = true;
            }
            Err(_) => self.connection_status = false,
        } */

        match self.send(commands::AT_commands::CWMODE(1)) {
            Ok(_) => {
                self.connection_status = true;
            }
            Err(_) => self.connection_status = false,
        }

        match self.send(commands::AT_commands::CIPMUX(0)) {
            Ok(_) => {
                self.connection_status = true;
            }
            Err(_) => self.connection_status = false,
        }

        match self.send(commands::AT_commands::CIPSTART_EXT(
            "UDP", "0.0.0.0", port, port, 2,
        )) {
            Ok(_) => {
                self.connection_status = true;
            }
            Err(_) => self.connection_status = false,
        }

        match self.send(commands::AT_commands::CIPSEND(4)) {
            Ok(_) => {
                self.connection_status = true;
            }
            Err(_) => self.connection_status = false,
        }

        /* match self.send(commands::AT_commands::SEND("TEST")) {
            Ok(_) => {
                self.connection_status = true;
            }
            Err(_) => self.connection_status = false,
        } */
        // Return
        if !self.connection_status {
            Err(())
        } else {
            Ok(())
        }
    }

    // TODO: Check for HOW we are connected to the network (CIPSERVER / TCP / UDP etc.)
    /// Sends data to the network
    pub fn send_data(&mut self, data: &str) -> Result<(), ()> {
        let mut chk;
        let len = data.len() as u16;
        match self.send(commands::AT_commands::CIPSEND(len)) {
            Ok(_) => {
                chk = true;
            }
            Err(_) => chk = false,
        }

        match self.send(commands::AT_commands::SEND(data)) {
            Ok(_) => {
                chk = true;
            }
            Err(_) => chk = false,
        }

        // Return
        if !chk {
            Err(())
        } else {
            Ok(())
        }
    }

    pub fn read_network_data(&mut self, mut buffer: &mut [u8]) -> Result<u8, ()> {
        let mut found_data: bool = false;
        let mut data_len: u8 = 0;
        while !found_data {
            let (cmd, len) = self.get_response(&mut buffer).unwrap();
            if cmd == commands::AT_response::IPD {
                found_data = true;
                data_len = len;
            }
        }
        Ok(data_len)
    }

    //------------------------------------------------------------------------
    // NON public functions
    //------------------------------------------------------------------------

    // Handels the sending of a specific function
    fn send(&mut self, mut cmd: commands::AT_commands) -> Result<(), ()> {
        self.send_command(&cmd);
        Ok(())
    }

    // Handles transporting the send_ to the module, and verifying the response from the module.
    fn send_command(&mut self, cmd: &commands::AT_commands) {
        let mut cmd_buffer: String<U64> = String::new();
        let mut expected_buffer: String<U64> = String::new();
        // reset buffers
        cmd_buffer.clear();
        expected_buffer.clear();

        let (send_, expected, endChar) = match cmd {
            commands::AT_commands::AT => ("AT", commands::AT_response::OK, true),
            commands::AT_commands::ATE(echo) => {
                if *echo == true {
                    ("ATE1", commands::AT_response::OK, true)
                } else {
                    ("ATE0", commands::AT_response::OK, true)
                }
            }
            commands::AT_commands::RST => ("AT+RST", commands::AT_response::ready, true),
            commands::AT_commands::CWJAP(ssid, pwd) => {
                write!(cmd_buffer, "AT+CWJAP=\"{}\",\"{}\"", ssid, pwd).unwrap();
                (cmd_buffer.as_str(), commands::AT_response::OK, true)
            }
            commands::AT_commands::CWMODE(mode) => {
                write!(cmd_buffer, "AT+CWMODE={}", mode).unwrap();
                (cmd_buffer.as_str(), commands::AT_response::OK, true)
            }
            commands::AT_commands::CIFSR => ("AT+CIFSR", commands::AT_response::OK, true),
            commands::AT_commands::CIPMUX(mode) => {
                write!(cmd_buffer, "AT+CIPMUX={}", mode).unwrap();
                (cmd_buffer.as_str(), commands::AT_response::OK, true)
            }
            commands::AT_commands::CIPSERVER(mode) => {
                write!(cmd_buffer, "AT+CIPSERVER={}", mode).unwrap();
                (cmd_buffer.as_str(), commands::AT_response::OK, true)
            }
            commands::AT_commands::CIPSERVER_EXT(mode, port) => {
                write!(cmd_buffer, "AT+CIPSERVER={},{}", mode, port).unwrap();
                (cmd_buffer.as_str(), commands::AT_response::OK, true)
            }
            commands::AT_commands::CIPSTART(protocol, remote_ip, remote_port) => {
                write!(
                    cmd_buffer,
                    "AT+CIPSTART=\"{}\",\"{}\",{}",
                    protocol, remote_ip, remote_port
                )
                .unwrap();
                (cmd_buffer.as_str(), commands::AT_response::OK, true)
            }
            commands::AT_commands::CIPSTART_EXT(
                protocol,
                remote_ip,
                remote_port,
                local_port,
                mode,
            ) => {
                write!(
                    cmd_buffer,
                    "AT+CIPSTART=\"{}\",\"{}\",{},{},{}",
                    protocol, remote_ip, remote_port, local_port, mode
                )
                .unwrap();
                (cmd_buffer.as_str(), commands::AT_response::OK, true)
            }
            commands::AT_commands::CIPSEND(length) => {
                write!(cmd_buffer, "AT+CIPSEND={}", length).unwrap();
                (cmd_buffer.as_str(), commands::AT_response::OK, true)
            }
            commands::AT_commands::SEND(data) => {
                write!(cmd_buffer, "{}", data).unwrap();
                (cmd_buffer.as_str(), commands::AT_response::OK, false)
            }
            _ => (
                "commands::AT_commands::NO_COMMAND",
                commands::AT_response::UNKNOWN_COMMAND,
                true,
            ),
        };

        let mut found_expected_resp = false;
        // Writes the send_ to the ESP device
        self.write_serial(send_.as_bytes(), endChar).ok();
        while !found_expected_resp {
            // Gets response from ESP
            let mut other: [u8; 64] = [0; 64];
            match self.get_response(&mut other) {
                Ok((cmd, len)) => {
                    if cmd == expected {
                        found_expected_resp = true;
                    } else if cmd == commands::AT_response::ERROR {
                        // Resend
                        self.write_serial(send_.as_bytes(), endChar).ok();
                    } else if cmd == commands::AT_response::ALREADY_CONNECTED {
                        found_expected_resp = true;
                    } else if cmd == commands::AT_response::WIFI_CONNECTED {
                        self.connection_status = true;
                    //self.delay.delay_ms(2000u16);
                    } else if cmd == commands::AT_response::WIFI_DISCONNECT {
                        self.connection_status = false;
                        self.got_ip = false;
                    //self.delay.delay_ms(2000u16);
                    } else if cmd == commands::AT_response::WIFI_GOT_IP {
                        self.got_ip = true;
                    } else {
                        found_expected_resp = false;
                        self.delay.delay_ms(200u16);
                        self.write_serial(send_.as_bytes(), endChar).ok();
                    }
                }
                Err(_) => found_expected_resp = false,
            }
        }
    }

    fn get_response(&mut self, mut data: &mut [u8]) -> Result<(commands::AT_response, u8), ()> {
        // Buffer for response from ESP device
        let mut buffer: [u8; 64] = [0; 64];
        let mut response: commands::AT_response = commands::AT_response::UNKNOWN_COMMAND;

        // Read from serial until
        self.read_serial(&mut buffer);
        /* while buffer[0] == 0 || (buffer[0] == b'\r' && buffer[1] == b'\n') {
            self.read_serial(&mut buffer).ok();
        } */

        /* // Find where the end of the message is
        let len = buffer.len();
        let mut index = 0;
        for i in 0..len {
            if buffer[i] == 0 {
                break;
            }
            index = index + 1;
        }
        // Break free the message for easier handling
        let mut message = buffer.split_at_mut(index).0; */

        // Find network data
        // TODO: bound check of ':'
        let mut data_len = 0;
        if buffer.starts_with(b"+IPD") {
            let mut index = 5;
            let mut num_digit = 0;
            while buffer[index] != b':' {
                index = index + 1;
                num_digit = num_digit + 1;
            }
            for i in 0..num_digit {
                data_len = data_len + (buffer[4 + num_digit - i] - 48) * 10u8.pow(i as u32);
            }
            //let m_data = buffer.split_at(index + 1).1;
            let mut new_index = 0;
            for i in (index + 1)..(index as usize + data_len as usize + 1)  {
                data[new_index] = buffer[i];
                new_index = new_index + 1;
            }
            //data = message.split_at_mut(index + 1).1;
            //let (m_cmd, m_length) = m_crap.split_at(4);

            response = commands::AT_response::IPD;
        } else {
            // Find the response
            if buffer.starts_with(b"OK") {
                response = commands::AT_response::OK;
            } else if buffer.starts_with(b"FAIL") {
                response = commands::AT_response::FAIL;
            } else if buffer.starts_with(b"ready") {
                response = commands::AT_response::ready;
            } else if buffer.starts_with(b"> ") {
                response = commands::AT_response::ready_to_send;
            } else if buffer.starts_with(b"Recv") {
                response = commands::AT_response::OK;
            } else if buffer.starts_with(b"ALREADY CONNECTED") {
                response = commands::AT_response::ALREADY_CONNECTED;
            } else if buffer.starts_with(b"WIFI CONNECTED") {
                response = commands::AT_response::WIFI_CONNECTED;
            } else if buffer.starts_with(b"WIFI GOT IP") {
                response = commands::AT_response::WIFI_GOT_IP;
            } else if buffer.starts_with(b"WIFI DISCONNECT") {
                response = commands::AT_response::WIFI_DISCONNECT;
            } else {
                response = commands::AT_response::UNKNOWN_COMMAND;
            }
        }

        Ok((response, data_len))
    }

    // Writes to the serial interface
    fn write_serial(&mut self, buffer: &[u8], endChar: bool) -> Result<(), E> {
        let len = buffer.len();
        for i in 0..len {
            block!(self.tx.write((buffer[i]).into()))?;
        }
        if endChar {
            // Send end characters
            block!(self.tx.write((b'\r').into()))?;
            block!(self.tx.write((b'\n').into()))?;
        }

        Ok(())
    }

    // Reads from the serial interface
    fn read_serial(&mut self, buffer: &mut [u8]) -> Result<(), ()> {
        let mut first_byte: u8 = 0;
        let mut parsed_first_byte: bool = false;
        while first_byte == 0 {
            if let Some(byte) = block!(self.rx.read()).ok() {
                first_byte = byte;
            }
            if first_byte == b'\r' {
                if let Some(byte) = block!(self.rx.read()).ok() {
                    parsed_first_byte = true;
                }
            }
        }

        // found start
        let mut missed_byte: u8 = 0;
        let mut parse_missed_byte: bool = false;
        for elem in buffer {
            if !parsed_first_byte {
                *elem = first_byte;
                parsed_first_byte = true;
            } else if parse_missed_byte {
                *elem = missed_byte;
                parse_missed_byte = false;
            } else {
                if let Some(byte) = block!(self.rx.read()).ok() {
                    if byte == b'\r' {
                        if let Some(byte) = block!(self.rx.read()).ok() {
                            if byte == b'\n' {
                                break;
                            } else {
                                missed_byte = byte;
                                parse_missed_byte = true;
                            }
                        }
                        break;
                    } else {
                        *elem = byte;
                    }
                } else {
                    return Err(());
                }
            }
        }
        Ok(())
    }
}
