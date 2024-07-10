use std::fs;
use std::io::{Cursor, Read, Seek, SeekFrom};

struct HeaderBytes {
    start_image: u16,
    application_header: u16,
    quantization_table: u16,
    start_frame: u16,
    huffman_table: u16,
    start_scan: u16,
    end_image: u16,
}
static HEADER_BYTES: HeaderBytes =  HeaderBytes {
    start_image: 0xffd8,
    application_header: 0xffe0,
    quantization_table: 0xffdb,
    start_frame: 0xffc0,
    huffman_table: 0xffc4,
    start_scan: 0xffda,
    end_image: 0xffd9,
};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let data = fs::read("sample1.jfif")?;
    let mut cursor = Cursor::new(data);
    print_application(&mut cursor);

    Ok(())
}


fn is_jpeg(cursor: &mut Cursor<Vec<u8>>) -> bool {
    let mut header = [0u8;2];
    read_position(0, cursor, &mut header);
    HEADER_BYTES.start_image == u16::from_be_bytes(header)
}

fn read_position(start: u64, cursor: &mut Cursor<Vec<u8>>, array: &mut [u8] ){
    match cursor.seek(SeekFrom::Start(start)){
       Err(error) => panic!("Problem seeking: {error:?}"),
        _ => (),
    };

    match cursor.read_exact(array){
        Err(error) => panic!("Problem seeking: {error:?}"),
        _ => (),
    };
}

fn print_application(cursor: &mut Cursor<Vec<u8>>){
    println!("-------------\n READING HEADER " );
    if(is_jpeg(cursor)){
        println!("Jpeg file")
    }else {
        panic!("No JPEG file")
    };

    let mut two_bytes = [0u8;2];
    let mut one_byte = [0u8;1];
    let mut five_bytes = [0u8;5];

    read_position(2, cursor, &mut two_bytes);
    println!("Marker: {:#x?} {:#x?}", two_bytes[0], two_bytes[1] );

    read_position(4, cursor, &mut two_bytes);
    let size =  u16::from_be_bytes(two_bytes);
    println!("Length: {size:?}" );

    print!("Identifier: ");
    read_position(6, cursor, &mut five_bytes);
    for i in 0..5{
        print!("{}",five_bytes[i] as char,);
    }
    println!();
    print!("Version: ");
    read_position(11, cursor, &mut one_byte);
    print!("{}.",one_byte[0]);
    read_position(12, cursor, &mut one_byte);
    println!("{}",one_byte[0]);

    read_position(13, cursor, &mut one_byte);
    println!("Units {}",one_byte[0]);

    read_position(14, cursor, &mut two_bytes);
    print!("Density {}", u16::from_be_bytes(two_bytes));
    read_position(16, cursor, &mut two_bytes);
    println!("x{}", u16::from_be_bytes(two_bytes));

    read_position(18, cursor, &mut one_byte);
    print!("Thumbnail {}", one_byte[0]);
    read_position(19, cursor, &mut one_byte);
    println!("x{}", one_byte[0]);
    println!("------------");
}