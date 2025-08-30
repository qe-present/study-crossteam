use std::io::{stdout,Write};
use crossterm::{
    execute,
    style::{ Color,Print, ResetColor, SetForegroundColor},
};
fn main() ->std::io::Result<()>{
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        Print("Hello, crossteam!"),
        ResetColor,
    )?;
    Ok(())
}