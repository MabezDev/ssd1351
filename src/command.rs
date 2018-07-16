/* TODO
    Is SPI the only interace to the ssd1351?
 */

pub enum Command {
    /// Column address
    ColumnAddress(u8,u8),
    /// Row address
    RowAddress(u8,u8)
}

impl Command {
    pub fn send(/* TODO interface */){

    }
}