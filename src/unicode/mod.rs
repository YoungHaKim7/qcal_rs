use std::io;

pub fn read_utf8_char() -> io::Result<char> {
    use std::io::Read;

    let mut buf = [0u8; 4];
    let mut stdin = std::io::stdin();

    // Read first byte
    stdin.read_exact(&mut buf[0..1])?;

    let width = utf8_char_width(buf[0]);

    if width == 1 {
        return Ok(buf[0] as char);
    }

    stdin.read_exact(&mut buf[1..width])?;

    let s = std::str::from_utf8(&buf[..width])
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"))?;

    Ok(s.chars().next().unwrap())
}

fn utf8_char_width(b: u8) -> usize {
    match b {
        0x00..=0x7F => 1,
        0xC0..=0xDF => 2,
        0xE0..=0xEF => 3,
        0xF0..=0xF7 => 4,
        _ => 1, // fallback
    }
}
