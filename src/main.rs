extern crate core;

use std::char::decode_utf16;
use std::io::{Cursor, Read, Write};
use std::net::TcpStream;
use byteorder::{BigEndian, ReadBytesExt};

fn main() {
    check_patch().unwrap();
}

fn check_patch() -> anyhow::Result<()> {
    // patch.path... - patch4.path...
    let mut stream = TcpStream::connect("patch3.pathofexile.com:12995")?;

    stream.write(&[1, 6])?;
    let mut read_buffer = [0; 2048];
    let result = stream.read(&mut read_buffer)?;

    let received_data = &read_buffer[0..result];

    let mut cursor = Cursor::new(received_data);
    // first byte should be 02
    if cursor.read_u8()? != 2 {
        panic!("Header value does not match")
    }

    // next 32 empty bytes
    cursor.set_position(33);

    let str_one = read_string(&mut cursor)?;
    println!("{:#?}", str_one);
    let str_two = read_string(&mut cursor)?;
    println!("{:#?}", str_two);

    Ok(())
}

fn read_string(mut c: &mut Cursor<&[u8]>) -> anyhow::Result<String> {
    let str_length = c.read_u16::<BigEndian>()?;
    let mut str_slice: Vec<u8> = vec![0; (str_length * 2) as usize];
    c.read(str_slice.as_mut_slice())?;


    Ok(
        rust_read_string(str_slice.as_slice(), str_length as usize)?
    )
}

fn rust_read_string(slice: &[u8], size: usize) -> anyhow::Result<String> {
    assert!(2*size <= slice.len());
    let iter = (0..size)
        .map(|i| u16::from_le_bytes([slice[2*i], slice[2*i+1]]));

    Ok(
        std::char::decode_utf16(iter).collect::<Result<String, _>>()?
    )
}