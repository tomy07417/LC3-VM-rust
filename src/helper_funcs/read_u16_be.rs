use std::io::Read;

/// Read a big-endian `u16` from the provided reader.
pub fn read_u16_be<R: Read>(r: &mut R) -> std::io::Result<u16> {
    let mut buf = [0u8; 2];
    r.read_exact(&mut buf)?;
    Ok(u16::from_be_bytes(buf))
}
