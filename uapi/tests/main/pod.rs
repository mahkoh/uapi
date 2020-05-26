use uapi::*;

#[test]
fn pod_read_() {
    let buf = [1u8, 0, 0, 0];
    let i: i32 = pod_read(&buf[..]).unwrap();
    assert_eq!(i.to_le(), 1);
}

#[test]
fn pod_read_init_() {
    let buf = [3u8, 0, 0, 0, 0, 0];
    let i: i32 = pod_read_init(&buf[..]).unwrap();
    assert_eq!(i.to_le(), 3);
}

#[test]
fn pod_write_() {
    let buf = [1u8, 0, 0, 0];
    let mut i: i32 = 0;
    pod_write(&buf[..], &mut i).unwrap();
    assert_eq!(i.to_le(), 1);
}

#[test]
fn pod_iter_() {
    let buf = [1u8, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0];
    assert_eq!(
        pod_iter::<i32, _>(&buf[..])
            .unwrap()
            .map(|v| v.to_le())
            .collect::<Vec<_>>(),
        [1, 2, 3]
    );
}

#[test]
fn as_bytes_() {
    assert_eq!(
        as_bytes(&[1u32.to_le(), 2, 3][..]),
        [1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]
    );
}
