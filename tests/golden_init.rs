//! Golden test: `init_sequence` must be byte-for-byte identical to the
//! known-good vendor bring-up sequence in `TDO_init_sequence.txt`.
//!
//! The reference blob — not the datasheet prose — is the ground truth for
//! this panel: the datasheet is internally inconsistent, while the blob is
//! what actually lights the panel up. Any refactor of the typed parameter
//! layer must leave this log unchanged.

use std::io;
use std::sync::{Arc, Mutex};

use st7701s::st7701s_spi::{
    device::ST7701S, protocol::connection::Connection, sequences::init::init_sequence,
};

type Log = Arc<Mutex<Vec<(u8, Vec<u8>)>>>;

/// A `Connection` that records every transmission instead of touching
/// hardware. Reads report zeroed data.
#[derive(Debug)]
struct RecordingConnection {
    log: Log,
}

impl Connection for RecordingConnection {
    fn command(&self, address: u8) -> io::Result<()> {
        self.log.lock().unwrap().push((address, Vec::new()));
        Ok(())
    }

    fn write(&self, address: u8, write_buffer: &[u8]) -> io::Result<()> {
        self.log
            .lock()
            .unwrap()
            .push((address, write_buffer.to_vec()));
        Ok(())
    }

    fn read(&self, address: u8, read_buffer: &mut [u8]) -> io::Result<()> {
        read_buffer.fill(0);
        self.log.lock().unwrap().push((address, Vec::new()));
        Ok(())
    }
}

/// `TDO_init_sequence.txt`, transcribed as (address, parameters) pairs in
/// transmission order. Commands without parameters record an empty slice.
const EXPECTED: &[(u8, &[u8])] = &[
    (0xFF, &[0x77, 0x01, 0x00, 0x00, 0x10]), // Command2 BK0
    (0xC0, &[0x3B, 0x00]),                   // LNESET
    (0xC1, &[0x0B, 0x02]),                   // PORCTRL
    (0xC2, &[0x00, 0x02]),                   // INVSEL
    (0xCC, &[0x10]),                         // undocumented
    (0xCD, &[0x08]),                         // COLCTRL
    (
        0xB0, // PVGAMCTRL
        &[
            0x02, 0x13, 0x1B, 0x0D, 0x10, 0x05, 0x08, 0x07, 0x07, 0x24, 0x04, 0x11, 0x0E, 0x2C,
            0x33, 0x1D,
        ],
    ),
    (
        0xB1, // NVGAMCTRL
        &[
            0x05, 0x13, 0x1B, 0x0D, 0x11, 0x05, 0x08, 0x07, 0x07, 0x24, 0x04, 0x11, 0x0E, 0x2C,
            0x33, 0x1D,
        ],
    ),
    (0xFF, &[0x77, 0x01, 0x00, 0x00, 0x11]), // Command2 BK1
    (0xB0, &[0x5D]),                         // VRHS
    (0xB1, &[0x43]),                         // VCOMS
    (0xB2, &[0x81]),                         // VGHSS
    (0xB3, &[0x80]),                         // TESTCMD
    (0xB5, &[0x43]),                         // VGLS
    (0xB7, &[0x85]),                         // PWCTRL1
    (0xB8, &[0x20]),                         // PWCTRL2
    (0xC1, &[0x78]),                         // SPD1
    (0xC2, &[0x78]),                         // SPD2
    (0xD0, &[0x88]),                         // MIPISET1
    (0xE0, &[0x00, 0x00, 0x02]),
    (
        0xE1,
        &[
            0x03, 0xA0, 0x00, 0x00, 0x04, 0xA0, 0x00, 0x00, 0x00, 0x20, 0x20,
        ],
    ),
    (0xE2, &[0x00; 13]),
    (0xE3, &[0x00, 0x00, 0x11, 0x00]),
    (0xE4, &[0x22, 0x00]),
    (
        0xE5,
        &[
            0x05, 0xEC, 0xA0, 0xA0, 0x07, 0xEE, 0xA0, 0xA0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
    ),
    (0xE6, &[0x00, 0x00, 0x11, 0x00]),
    (0xE7, &[0x22, 0x00]),
    (
        0xE8,
        &[
            0x06, 0xED, 0xA0, 0xA0, 0x08, 0xEF, 0xA0, 0xA0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
    ),
    (0xEB, &[0x00, 0x00, 0x40, 0x40, 0x00, 0x00, 0x00]),
    (
        0xED,
        &[
            0xFF, 0xFF, 0xFF, 0xBA, 0x0A, 0xBF, 0x45, 0xFF, 0xFF, 0x54, 0xFB, 0xA0, 0xAB, 0xFF,
            0xFF, 0xFF,
        ],
    ),
    (0xEF, &[0x10, 0x0D, 0x04, 0x08, 0x3F, 0x1F]),
    (0xFF, &[0x77, 0x01, 0x00, 0x00, 0x13]), // Command2 BK3
    (0xEF, &[0x08]),
    (0xFF, &[0x77, 0x01, 0x00, 0x00, 0x00]), // Command2 disable
    (0x11, &[]),                             // SLPOUT
    (0x29, &[]),                             // DISPON
    (0x36, &[0x00]),                         // MADCTL
    (0x3A, &[0x60]),                         // COLMOD
];

#[test]
fn init_sequence_matches_tdo_reference_bytes() {
    let log: Log = Arc::default();
    let connection = RecordingConnection {
        log: Arc::clone(&log),
    };

    let display = ST7701S::<_>::new(connection);
    init_sequence(display).expect("recording connection never fails");

    let log = log.lock().unwrap().clone();

    for (index, (actual, expected)) in log.iter().zip(EXPECTED).enumerate() {
        assert_eq!(
            (actual.0, actual.1.as_slice()),
            *expected,
            "transmission {index} (register {:#04X}) does not match the reference sequence",
            expected.0,
        );
    }

    assert_eq!(
        log.len(),
        EXPECTED.len(),
        "transmission count differs from the reference sequence",
    );
}
