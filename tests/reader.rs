use kaitai::*;

#[test]
fn basic_strip_right() {
    let b = [1, 2, 3, 4, 5, 5, 5, 5];
    let reader = BytesReader::new(&b[..]);
    let c = reader.bytes_strip_right(&b, 5);

    assert_eq!([1, 2, 3, 4], c);
}

#[test]
fn basic_read_bytes() {
    let b = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let reader = BytesReader::new(&b[..]);

    assert_eq!(reader.read_bytes(4).unwrap(), &[1, 2, 3, 4]);
    assert_eq!(reader.read_bytes(3).unwrap(), &[5, 6, 7]);
    assert_eq!(
        reader.read_bytes(4).unwrap_err(),
        KError::Incomplete(Needed::Size(3))
    );
    assert_eq!(reader.read_bytes(1).unwrap(), &[8]);
}

#[test]
fn read_bits_single() {
    let b = vec![0x80];
    let reader = BytesReader::new(&b[..]);

    assert_eq!(reader.read_bits_int_be(1).unwrap(), 1);
}

#[test]
fn read_bits_multiple() {
    // 0xA0
    let b = vec![0b10100000];
    let reader = BytesReader::new(&b[..]);

    assert_eq!(reader.read_bits_int_be(1).unwrap(), 1);
    assert_eq!(reader.read_bits_int_be(1).unwrap(), 0);
    assert_eq!(reader.read_bits_int_be(1).unwrap(), 1);
}

#[test]
fn read_bits_large() {
    let b = vec![0b10100000];
    let reader = BytesReader::new(&b[..]);

    assert_eq!(reader.read_bits_int_be(3).unwrap(), 5);
}

#[test]
fn read_bits_span() {
    let b = vec![0x01, 0x80];
    let reader = BytesReader::new(&b[..]);

    assert_eq!(reader.read_bits_int_be(9).unwrap(), 3);
}

#[test]
fn read_bits_too_large() {
    let b: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let reader = BytesReader::new(&b[..]);

    assert_eq!(
        reader.read_bits_int_be(65).unwrap_err(),
        KError::ReadBitsTooLarge { requested: 65 }
    )
}

#[test]
fn read_bytes_term() {
    let b = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let reader = BytesReader::new(&b[..]);

    assert_eq!(reader.read_bytes_term(3, false, false, false).unwrap(), &[1, 2]);
    assert_eq!(reader.read_bytes_term(3, true, false, true).unwrap(), &[3]);
    assert_eq!(reader.read_bytes_term(3, false, true, true).unwrap(), &[]);
    assert_eq!(reader.read_bytes_term(5, true, true, true).unwrap(), &[4, 5]);
    assert_eq!(reader.read_bytes_term(8, false, false, true).unwrap(), &[6, 7]);
    assert_eq!(reader.read_bytes_term(11, false, true, true).unwrap_err(), KError::EncounteredEOF);
    assert_eq!(reader.read_bytes_term(9, true, true, false).unwrap(), &[8, 9]);
    assert_eq!(reader.read_bytes_term(10, true, false, false).unwrap(), &[10]);
}

#[test]
fn process_xor_one() {
    let b = vec![0x66];
    let reader = BytesReader::new(&b[..]);
    fn as_stream_trait<S: KStream>(_io: &S) {
        let res = S::process_xor_one(_io.read_bytes(1).unwrap(), 3);
        assert_eq!(0x65, res[0]);
    }
    as_stream_trait(&reader);
}

#[test]
fn process_xor_many() {
    let b = vec![0x66, 0x6F];
    let reader = BytesReader::new(&b[..]);
    fn as_stream_trait<S: KStream>(_io: &S) {
        let key : Vec<u8> = vec![3, 3];
        let res = S::process_xor_many(_io.read_bytes(2).unwrap(), &key);
        assert_eq!(vec![0x65, 0x6C], res);
    }
    as_stream_trait(&reader);
}

#[test]
fn process_rotate_left() {
    let b = vec![0x09, 0xAC];
    let reader = BytesReader::new(&b[..]);
    fn as_stream_trait<S: KStream>(_io: &S) {
        let res = S::process_rotate_left(_io.read_bytes(2).unwrap(), 3);
        let expected : Vec<u8> = vec![0x48, 0x65];
        assert_eq!(expected, res);
    }
    as_stream_trait(&reader);
}

#[test]
fn basic_seek() {
    let b = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let reader = BytesReader::new(&b[..]);

    assert_eq!(reader.read_bytes(4).unwrap(), &[1, 2, 3, 4]);
    let pos = reader.pos();
    reader.seek(1).unwrap();
    assert_eq!(reader.read_bytes(4).unwrap(), &[2, 3, 4, 5]);
    reader.seek(pos).unwrap();
    assert_eq!(reader.read_bytes(4).unwrap(), &[5, 6, 7, 8]);
    assert_eq!(reader.seek(9).unwrap_err(),
               KError::Incomplete(Needed::Size(1)));
}
