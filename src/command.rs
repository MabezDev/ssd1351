use display_interface::{DataFormat, DisplayError, WriteOnlyDataCommand};

const REMAP_BASE: u8 = 0b00100100;

pub enum Command {
    /// Column address
    Column(u8, u8),
    /// Row address
    Row(u8, u8),
    /// CommandLock
    CommandLock(u8),
    /// DisplayOn
    DisplayOn(bool),
    /// WriteRam
    WriteRam,
    /// ClockDiv
    ClockDiv(u8),
    /// MuxRatio
    MuxRatio(u8),
    /// SetRemap - horizontal or vertical increment, SegmentRemap, Reverse scan direction
    SetRemap(bool, bool, bool),
    /// Display Start Row
    StartLine(u8),
    /// DisplayOffset
    DisplayOffset(u8),
    /// SetGpio
    SetGpio(u8),
    /// FunctionSelect
    FunctionSelect(u8),
    /// PreCharge
    PreCharge(u8),
    /// Vcomh
    Vcomh(u8),
    /// NormalDisplayOn
    Invert(bool),
    /// Contrast
    Contrast(u8),
    /// ContrastMaster
    ContrastCurrent(u8),
    /// SetVsl
    SetVsl,
    /// SetPrecharge
    PreCharge2(u8),
    // PhaseLength(u8)
}

impl Command {
    /// Send command to SSD1351
    pub fn send<DI>(self, iface: &mut DI) -> Result<(), DisplayError>
    where
        DI: WriteOnlyDataCommand,
    {
        // Transform command into a fixed size array of 7 u8 and the real length for sending
        // TODO can we replace the use if the static buffers?
        let (command, data, len) = match self {
            Command::CommandLock(val) => (0xFD, [val, 0, 0, 0, 0, 0], 1),
            Command::DisplayOn(val) => (if val { 0xAF } else { 0xAE }, [0, 0, 0, 0, 0, 0], 0),
            Command::ClockDiv(val) => (0xB3, [val, 0, 0, 0, 0, 0], 1),
            Command::MuxRatio(val) => (0xCA, [val, 0, 0, 0, 0, 0], 1),
            Command::SetRemap(incr, remap, scan) => (
                0xA0,
                [
                    REMAP_BASE | (incr as u8) | (remap as u8) << 1 | (scan as u8) << 4,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                1,
            ),
            Command::Column(start, end) => (0x15, [start, end, 0, 0, 0, 0], 2),
            Command::Row(start, end) => (0x75, [start, end, 0, 0, 0, 0], 2),
            Command::StartLine(val) => (0xA1, [val, 0, 0, 0, 0, 0], 1),
            Command::DisplayOffset(val) => (0xA2, [val, 0, 0, 0, 0, 0], 1),
            Command::SetGpio(val) => (0xB5, [val, 0, 0, 0, 0, 0], 1),
            Command::FunctionSelect(val) => (0xAB, [val, 0, 0, 0, 0, 0], 1),
            Command::PreCharge(val) => (0xB1, [val, 0, 0, 0, 0, 0], 1),
            Command::Vcomh(val) => (0xBE, [val, 0, 0, 0, 0, 0], 1),
            Command::Invert(val) => (if val { 0xA7 } else { 0xA6 }, [0, 0, 0, 0, 0, 0], 0),
            Command::Contrast(val) => (0xC1, [0xC8, val, 0xC8, 0, 0, 0], 3),
            Command::ContrastCurrent(val) => (0xC7, [val, 0, 0, 0, 0, 0], 1),
            Command::SetVsl => (0xB4, [0xA0, 0xB5, 0x55, 0, 0, 0], 3),
            Command::PreCharge2(val) => (0xB6, [val, 0, 0, 0, 0, 0], 1),
            Command::WriteRam => (0x5C, [0, 0, 0, 0, 0, 0], 0),
        };

        // Send command over the interface
        iface.send_commands(DataFormat::U8(&[command]))?;

        if len > 0 {
            iface.send_data(DataFormat::U8(&data[0..len]))?;
        }

        Ok(())
    }
}

// Transform command into a fixed size array of 7 u8 and the real length for sending
// let (data, len) = match self {
//     Command::Contrast(val) => ([0x81, val, 0, 0, 0, 0, 0], 2),
//     Command::AllOn(on) => ([0xA4 | (on as u8), 0, 0, 0, 0, 0, 0], 1),
//     Command::Invert(inv) => ([0xA6 | (inv as u8), 0, 0, 0, 0, 0, 0], 1),
//     Command::DisplayOn(on) => ([0xAE | (on as u8), 0, 0, 0, 0, 0, 0], 1),
//     Command::HScrollSetup(dir, start, end, rate) => (
//         [
//             0x26 | (dir as u8),
//             0,
//             start as u8,
//             rate as u8,
//             end as u8,
//             0,
//             0xFF,
//         ],
//         7,
//     ),
//     Command::VHScrollSetup(dir, start, end, rate, offset) => (
//         [
//             0x28 | (dir as u8),
//             0,
//             start as u8,
//             rate as u8,
//             end as u8,
//             offset,
//             0,
//         ],
//         6,
//     ),
//     Command::EnableScroll(en) => ([0x2E | (en as u8), 0, 0, 0, 0, 0, 0], 1),
//     Command::VScrollArea(above, lines) => ([0xA3, above, lines, 0, 0, 0, 0], 3),
//     Command::LowerColStart(addr) => ([0xF & addr, 0, 0, 0, 0, 0, 0], 1),
//     Command::UpperColStart(addr) => ([0x1F & addr, 0, 0, 0, 0, 0, 0], 1),
//     Command::AddressMode(mode) => ([0x20, mode as u8, 0, 0, 0, 0, 0], 2),
//     Command::ColumnAddress(start, end) => ([0x21, start, end, 0, 0, 0, 0], 3),
//     Command::PageAddress(start, end) => ([0x22, start as u8, end as u8, 0, 0, 0, 0], 3),
//     Command::PageStart(page) => ([0xB0 | (page as u8), 0, 0, 0, 0, 0, 0], 1),
//     Command::StartLine(line) => ([0x40 | (0x3F & line), 0, 0, 0, 0, 0, 0], 1),
//     Command::SegmentRemap(remap) => ([0xA0 | (remap as u8), 0, 0, 0, 0, 0, 0], 1),
//     Command::Multiplex(ratio) => ([0xA8, ratio, 0, 0, 0, 0, 0], 2),
//     Command::ReverseComDir(rev) => ([0xC0 | ((rev as u8) << 3), 0, 0, 0, 0, 0, 0], 1),
//     Command::DisplayOffset(offset) => ([0xD3, offset, 0, 0, 0, 0, 0], 2),
//     Command::ComPinConfig(alt, lr) => (
//         [
//             0xDA,
//             0x2 | ((alt as u8) << 4) | ((lr as u8) << 5),
//             0,
//             0,
//             0,
//             0,
//             0,
//         ],
//         2,
//     ),
//     Command::DisplayClockDiv(fosc, div) => {
//         ([0xD5, ((0xF & fosc) << 4) | (0xF & div), 0, 0, 0, 0, 0], 2)
//     }
//     Command::PreChargePeriod(phase1, phase2) => (
//         [0xD9, ((0xF & phase2) << 4) | (0xF & phase1), 0, 0, 0, 0, 0],
//         2,
//     ),
//     Command::VcomhDeselect(level) => ([0xDB, (level as u8) << 4, 0, 0, 0, 0, 0], 2),
//     Command::Noop => ([0xE3, 0, 0, 0, 0, 0, 0], 1),
//     Command::ChargePump(en) => ([0x8D, 0x10 | ((en as u8) << 2), 0, 0, 0, 0, 0], 2),
// };
