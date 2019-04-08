// Not all AT commands support all four variations mentioned above.
// Square brackets [ ] designate the default value; it is either not required or may not appear.
// String values need to be included in double quotation marks, for example: AT+CWSAP="ESP756290","21030826", 1,4
// The default baud rate is 115200.
// AT commands have to be capitalized, and must end with a new line (CR LF).

/// AT command types
pub enum AT_type {
    /// ```
    /// Queries the Set Commands internal parameters and their range of values
    /// ```
    Test, // AT+<x>=?
    /// ```
    /// Returns the current value of parameters
    /// ```
    Query, // AT+<x>?
    /// ```
    /// Sets the value of user-defined parameters in commands, and runs these commands
    /// ```
    Set, // AT+<x>=<...>
    /// ```
    /// Runs commands with no user-defined parameters
    /// ```
    Execute, // AT+<x>
}

/// AT commands
pub enum AT_commands<'a> {
    NO_COMMAND,
    // Basic
    /// ```
    /// Tests AT startup
    /// ```
    AT,
    /// ```
    /// Restarts the module
    /// ```
    RST,
    /// ```
    /// Checks version information
    /// ```
    GMR,
    /// ```
    /// Enters Deep-sleep mode
    /// GSLP(time)
    /// time:       The sleep duration in ms.
    /// ```
    GSLP(u16),
    /// ```
    /// Configures echoing of AT commands
    /// ATE(echo)
    /// echo:   true: Echo ON
    ///         false: Echo OFF
    /// ```   
    ATE(bool),
    /// ```
    /// Restores the factory default settings of the module
    /// ```
    RESTORE,
    /// ```
    /// The current UART configuration
    /// ```
    UART,
    /// ```
    /// Configures the sleep modes
    /// ```
    SLEEP,
    /// ```
    /// Configures a GPIO to wake ESP8266 up from Light-sleep mode
    /// ```
    WAKEUPGPIO,
    /// ```
    /// Sets the maximum value of the RF TX Power
    /// ```
    RFPOWER,
    /// ```
    /// Sets the RF TX Power according to VDD33
    /// ```
    RFVDD,
    /// ```
    /// Checks the available RAM size
    /// ```
    SYSRAM,
    /// ```
    /// Checks the ADC value
    /// ```
    SYSADC,
    /// ```
    /// Sets configuration of IO pins
    /// ```
    SYSIOSETCFG,
    /// ```
    /// Gets configuration of IO pins
    /// ```
    SYSIOGETCFG,
    /// ```
    /// Configures the direction of GPIO
    /// ```
    SYSGPIODIR,
    /// ```
    /// Configures the GPIO output level
    /// ```
    SYSGPIOWRITE,
    /// ```
    /// Checks the GPIO input level
    /// ```
    SYSGPIOREAD,
    /// ```
    /// System messages
    /// ```
    SYSMSG,

    // WiFi
    /// ```
    /// Sets the Current Wi-Fi mode. Configuration Not Saved in the Flash
    /// CWMODE(mode)
    /// - mode:     1: Station mode
    ///             2: SoftAP mode
    ///             3: SoftAP+Station mode
    /// ```
    CWMODE(u8),

    /// ```
    /// Connects to an AP. Configuration Not Saved in the Flash
    /// CWJAP(ssid, password)
    /// ```
    CWJAP(&'a str, &'a str),

    //CWLAPOPT,
    //CWLAP,
    /// ```
    /// Disconnects from the AP
    /// ```
    CWQAP,

    /// ```
    /// Configures the ESP8266 SoftAP. Configuration Not Saved in the Flash
    /// CWSAP(ssid, password, channel, encryption)
    /// - channel:      channel ID
    /// - encryption:   0: OPEN
    ///                 1: WPA_PSK
    ///                 2: WPA2_PSK
    ///                 3: WPA_WPA2_PSK
    /// ```
    CWSAP(&'a str, &'a str, u8, u8),

    //CWLIF,
    /// ```
    /// Enables/Disables DHCP. Configuration Not Saved in the Flash
    /// CWDHCP(mode, enable)
    /// - mode:     0: Sets ESP8266 SoftAP
    ///             1: Sets ESP8266 Station
    ///             2: Sets both SoftAP and Station
    /// - enable:   0: Disables DHCP
    ///             1: Enables DHCP
    /// ```
    CWDHCP(u8, u8),

    //CWDHCPS,
    /// ```
    /// Auto-Connects to the AP or Not
    /// CWAUTOCONN(enable)
    /// enable:  0: Does NOT auto-connect to AP on power-up
    ///          1: Connects to AP automatically on power-up
    /// ```
    CWAUTOCONN(u8),

    //CIPSTAMAC,
    //CIPAPMAC,
    //CIPSTA,
    //CIPAP,
    //CWSTARTSMART,
    //CWSTOPSMART,
    //CWSTARTDISCOVER,
    //CWSTOPDISCOVER,
    //WPS,
    //MDNS,
    /// ```
    /// Configures the Name of ESP8266 Station
    /// CWHOSTNAME(hostname)
    /// ```
    CWHOSTNAME(&'a str),

    //CWCOUNTRY,

    // TCP/IP

    //CIPSTATUS,
    //CIPDOMAIN,
    /// ```
    /// Establishes TCP Connection, UDP Transmission or SSL Connection
    /// CIPSTART(type, remote IP, remote port)
    /// type:        "TCP": Connection type TCP
    ///              "UDP": Connection type UDP
    ///              "SSL": Connection type SSL
    /// remote IP:   String parameter indicating the remote IP address
    /// remote port: The remote port number
    /// ```
    CIPSTART(&'a str, &'a str, u16),

    /// ```
    /// Establishes TCP Connection, UDP Transmission or SSL Connection (UDP WORKS. NOT FUNCTIONAL FOR ALL PROTOCOLS YET)
    /// CIPSTART_EXT(type, remote IP, remote port, local port, UDP mode)
    /// type:           "TCP": Connection type TCP
    ///                 "UDP": Connection type UDP
    ///                 "SSL": Connection type SSL
    /// remote IP:      String parameter indicating the remote IP address
    /// remote port:    The remote port number
    /// local port:     The local port to listen to
    /// UDP mode:       0: The destination peer entity of UDP will not change; this is the default setting
    ///                 1: The destination peer entity of UDP can change once
    ///                 2: The destination peer entity of UDP is allowed to change
    /// ```
    CIPSTART_EXT(&'a str, &'a str, u16, u16, u8),

    //CIPSSLSIZE,
    //CIPSSLCONF,
    /// ```
    /// Sends length of data
    /// CIPSEND(length)
    /// length:  Length of data to be sent
    /// ```
    CIPSEND(u16),

    /// ```
    /// Sends data
    /// SEND(data)
    /// data:  Data to be sent
    /// ```
    SEND(&'a str),

    //CIPSENDEX,
    //CIPSENDBUF,
    //CIPBUFRESET,
    //CIPBUFSTATUS,
    //CIPCHECKSEQ,
    /// ```
    /// Closes the TCP/UDP/SSL Connection
    /// CIPCLOSE(link ID)
    /// ```
    CIPCLOSE,

    /// ```
    /// Gets the Local IP Address
    /// CIFSR
    /// ```
    CIFSR,

    /// ```
    /// Enable or Disable Multiple Connections
    /// CIPMUX(mode)
    /// mode:        0: Single connection
    ///              1: Multiple connection
    /// ```
    CIPMUX(u8),

    /// ```
    /// Enable or Disable Multiple Connections
    /// CIPSERVER(mode)
    /// mode:        0: Deletes server
    ///              1: Creates server
    /// ```
    CIPSERVER(u8),

    /// ```
    /// Enable or Disable Multiple Connections
    /// CIPSERVER_EXT(mode, port)
    /// mode:        0: Deletes server
    ///              1: Creates server
    /// port:        Port number. 333 by default
    /// ```
    CIPSERVER_EXT(u8, u16),

    CIPSERVERMAXCONN,
    CIPMODE,
    SAVETRANSLINK,
    CIPSTO,

    /// ```
    /// Ping Packets
    /// PING(url)
    /// url:         IP address or url
    /// ```
    PING(&'a str),
    CIUPDATE,

    /// ```
    /// Shows the Remote IP and Port with +IPD
    /// CIPDINFO(mode)
    /// mode:        0: Does not show the remote IP and port with +IPD
    ///              1: Shows the remote IP and port with +IPD.
    /// ```
    CIPDINFO(u8),

    IPD, // NO prefix
    CIPRECVMODE,
    CIPRECVDATA,
    CIPRECVLEN,
    CIPSNTPCFG,
    CIPSNTPTIME,
    CIPDNS,
}

/// AT responses
#[derive(PartialEq)]
pub enum AT_response {
    UNKNOWN_COMMAND,
    OK,
    FAIL,
    ready,
    ERROR,
    ALREADY_CONNECTED,
    ready_to_send,
    WIFI_CONNECTED,
    WIFI_GOT_IP,
    WIFI_DISCONNECT,
    busy_s,
    busy_p,
    X_CONNECT,
    X_CLOSED,
    IPD,
    STA_CONNECTED,
    DIST_STA_IP,
    STA_DISCONNECTED,
}

/// Converts the response from the ESP device from &[u8] to enum for ease of use.
pub fn str_to_response(response: &[u8]) -> AT_response {
    match response {
        b"OK" => return AT_response::OK,
        b"FAIL" => return AT_response::FAIL,
        b"ready" => return AT_response::ready,
        b"ERROR" => return AT_response::ERROR,
        b"ALREADY CONNECTED" => return AT_response::ALREADY_CONNECTED,
        b">" => return AT_response::ready_to_send,
        b"WIFI CONNECTED" => return AT_response::WIFI_CONNECTED,
        b"WIFI GOT IP" => return AT_response::WIFI_GOT_IP,
        b"WIFI DISCONNECT" => return AT_response::WIFI_DISCONNECT,
        b"busy s..." => return AT_response::busy_s,
        b"busy p..." => return AT_response::busy_p,
        b",CONNECT" => return AT_response::X_CONNECT,
        b",CLOSED" => return AT_response::X_CLOSED,
        b"+IPD" => return AT_response::IPD,
        b"+STA_CONNECTED:" => return AT_response::STA_CONNECTED,
        b"+DIST_STA_IP:" => return AT_response::DIST_STA_IP,
        b"+STA_DISCONNECTED:" => return AT_response::STA_DISCONNECTED,
        _ => return AT_response::UNKNOWN_COMMAND,
    }
}

/// Converts the response from the ESP device from &[u8] to enum for ease of use.
pub fn response_to_str<'a>(response: AT_response) -> &'a str {
    match response {
        AT_response::OK => return "OK",
        AT_response::FAIL => return "FAIL",
        AT_response::ready => return "ready",
        AT_response::ERROR => return "ERROR",
        AT_response::ALREADY_CONNECTED => return "ALREADY CONNECTED",
        AT_response::ready_to_send => return ">",
        AT_response::WIFI_CONNECTED => return "WIFI CONNECTED",
        AT_response::WIFI_GOT_IP => return "WIFI GOT IP",
        AT_response::WIFI_DISCONNECT => return "WIFI DISCONNECT",
        AT_response::busy_s => return "busy s...",
        AT_response::busy_p => return "busy p...",
        AT_response::X_CONNECT => return ",CONNECT",
        AT_response::X_CLOSED => return ",CLOSED",
        AT_response::IPD => return "+IPD",
        AT_response::STA_CONNECTED => return "+STA_CONNECTED:",
        AT_response::DIST_STA_IP => return "+DIST_STA_IP:",
        AT_response::STA_DISCONNECTED => return "+STA_DISCONNECTED:",
        AT_response::UNKNOWN_COMMAND => return "UNKNOWN COMMAND",
        _ => return "UNREACHABLE!",
    }
}
