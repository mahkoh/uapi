use std::{
    borrow::Borrow,
    convert::{TryFrom, TryInto},
    ffi::{CStr, CString, OsStr, OsString},
    path::{Path, PathBuf},
};
use uapi::*;

fn ar<T: ?Sized, U: AsRef<T> + ?Sized>(t: &U) -> &T {
    t.as_ref()
}

fn am<T: ?Sized, U: AsMut<T> + ?Sized>(t: &mut U) -> &mut T {
    t.as_mut()
}

fn from<T: From<U>, U>(u: U) -> T {
    u.into()
}

fn try_from<T: TryFrom<U>, U>(u: U) -> std::result::Result<T, T::Error> {
    u.try_into()
}

#[test]
fn bstr() {
    assert_eq!(Bstr::empty(), "");
    assert_eq!(Bstr::empty(), Bstr::from_str(""));
    assert_eq!(Bstr::from_str("abc"), Bstr::from_bytes(b"abc"));
    assert_eq!(Bstr::from_path(Path::new("abc")), Bstr::from_bytes(b"abc"));
    assert_eq!(Bstr::from_os_str("abc".as_ref()), Bstr::from_bytes(b"abc"));
    assert_eq!(
        Bstr::from_bytes_mut(&mut [b'a', b'b', b'c']),
        Bstr::from_bytes(b"abc")
    );
    assert_eq!(
        Bstr::from_bytes_mut(&mut [b'a', b'b', b'c']).as_bytes_mut(),
        b"abc"
    );
    assert_eq!(&**Bstr::from_bytes_mut(&mut [b'a', b'b', b'c']), b"abc");
    assert_eq!(&**Bstr::from_bytes(b"abc"), b"abc");
    assert_eq!(Bstr::from_bytes(b"abc").as_str(), Ok("abc"));
    assert_eq!(Bstr::from_bytes(b"abc").as_os_str(), OsStr::new("abc"));
    assert_eq!(Bstr::from_bytes(b"abc").as_path(), Path::new("abc"));
    assert_eq!(Bstr::from_bytes(b"abc").len(), 3);
    assert!(Bstr::from_bytes(b"").is_empty());
    assert_eq!(Bstr::from_bytes(b"abc").to_ustring(), format_ustr!("abc"));
    assert_eq!(Bstr::from_bytes(b"abc").to_owned(), format_ustr!("abc"));

    let buf = &mut [b'a', b'b', b'c'];

    assert_eq!(Bstr::from_bytes(buf).as_ptr(), buf.as_ptr() as _);
    assert_eq!(
        Bstr::from_bytes_mut(buf).as_mut_ptr(),
        buf.as_mut_ptr() as _
    );

    assert_eq!(am::<[u8], _>(Bstr::from_bytes_mut(buf)), ustr!("abc"));
    assert_eq!(ar::<[u8], _>(Bstr::from_bytes(b"abc")), ustr!("abc"));
    assert_eq!(ar::<OsStr, _>(Bstr::from_bytes(b"abc")), ustr!("abc"));
    assert_eq!(ar::<Path, _>(Bstr::from_bytes(b"abc")), ustr!("abc"));
    assert_eq!(Bstr::from_bytes(b"abc").bytes(), ustr!("abc"));
    assert_eq!(format!("{:?}", Bstr::from_bytes(b"abc")), "\"abc\"");
    assert_eq!(format!("{}", Bstr::from_bytes(b"abc").display()), "abc");

    {
        let buf = &mut [b'a', b'b', b'c'];
        let bs = Bstr::from_bytes_mut(buf);
        bs[0] = b'd';
        assert_eq!(bs, "dbc");
    }

    assert_eq!(ar::<Bstr, _>(ustr!("abc").as_c_str().unwrap()), "abc");
}

#[test]
fn ustr() {
    assert_eq!(Ustr::empty(), "");
    assert_eq!(Ustr::empty(), Bstr::from_str(""));
    assert_eq!(Ustr::null(), "");
    assert!(Ustr::null().is_null());
    assert_eq!(Ustr::from_str("abc\0").unwrap(), "abc");
    assert_eq!(Ustr::from_str("abc"), None);
    unsafe {
        assert_eq!(
            Ustr::from_bytes_unchecked(&mut [b'a', b'b', b'c', 0]),
            "abc"
        );
    }
    assert_eq!(Ustr::from_bytes(b"abc\0").unwrap(), "abc");
    assert_eq!(Ustr::from_bytes(b"abc"), None);
    assert_eq!(Ustr::from_bytes(b"abc\0").unwrap().as_bytes(), b"abc");
    assert_eq!(
        Ustr::from_bytes(b"abc\0").unwrap().as_bytes_with_nul(),
        b"abc\0"
    );
    assert_eq!(
        Ustr::from_bytes(b"abc\0").unwrap().as_bytes_with_nul(),
        b"abc\0"
    );
    assert_eq!(
        Ustr::from_c_str(CStr::from_bytes_with_nul(b"abc\0").unwrap()),
        "abc"
    );
    {
        let buf = &mut [b'a', b'b', b'c', 0][..];
        assert_eq!(Ustr::from_bytes_mut(buf).unwrap(), "abc");
        assert_eq!(Ustr::from_bytes_mut(&mut []), None);
        unsafe {
            assert_eq!(Ustr::from_ptr(buf.as_ptr() as _), "abc");
        }
        unsafe {
            assert_eq!(Ustr::from_ptr_mut(buf.as_mut_ptr() as _), "abc");
        }
        // assert_eq!(Ustr::from_bytes_mut(buf).as_b_(), "abc");
    }
    assert_eq!(Ustr::from_os_str(OsStr::new("abc\0")).unwrap(), "abc");
    assert_eq!(Ustr::from_os_str(OsStr::new("abc")), None);
    assert_eq!(Ustr::from_path(Path::new("abc\0")).unwrap(), "abc");
    assert_eq!(Ustr::from_path(Path::new("abc")), None);
    assert_eq!(ustr!("abc").as_os_str_with_nul(), "abc\0");
    assert_eq!(ustr!("abc").as_path_with_nul().as_os_str(), "abc\0");
    assert!(!Ustr::empty().as_ptr_null().is_null());
    assert!(Ustr::null().as_ptr_null().is_null());
    assert_eq!(ustr!("abc").to_owned(), format!("abc"));

    let mut buf = &mut [b'a', b'b', b'c', 0][..];
    assert_eq!(
        am::<[u8], _>(Ustr::from_bytes_mut(buf).unwrap()),
        ustr!("abc")
    );
    assert_eq!(ar::<[u8], _>(ustr!("abc")), ustr!("abc"));
    assert_eq!(ar::<OsStr, _>(ustr!("abc")), ustr!("abc"));
    assert_eq!(ar::<Path, _>(ustr!("abc")), ustr!("abc"));
    assert_eq!(
        ar::<Ustr, _>(CStr::from_bytes_with_nul(b"abc\0").unwrap()),
        ustr!("abc")
    );

    assert_eq!(format!("{:?}", ustr!("abc")), "\"abc\"");
}

#[test]
fn ustring() {
    {
        let mut us = Ustring::new();
        assert_eq!(us.len(), 0);
        assert_eq!(us.capacity(), 0);
        us.reserve(1);
        assert_eq!(us.len(), 0);
        assert!(us.capacity() >= 1);
        us.reserve_exact(31);
        assert_eq!(us.len(), 0);
        assert!(us.capacity() >= 31);
        unsafe {
            us.with_unused(|b| {
                assert!(b.len() >= 31);
                b[0] = b'1';
                Ok(1)
            })
            .unwrap();
        }
        assert_eq!(us.len(), 1);
        assert_eq!(&us, "1");
        unsafe {
            let _ = us.with_unused(|b| {
                (0..b.len()).for_each(|i| b[i] = 1);
                Err(Errno(0))
            });
        }
        unsafe {
            assert_eq!(Ustr::from_ptr(us.as_ptr()), "1");
        }
    }

    assert_eq!(&Ustring::from_vec(vec!()), "");
    assert_eq!(&Ustring::from_vec_with_nul(vec!()).unwrap(), "");
    assert_eq!(&Ustring::from_vec_with_nul(vec!(b'1', 0)).unwrap(), "1");
    unsafe {
        assert_eq!(&Ustring::from_vec_with_nul_unchecked(vec!()), "");
        assert_eq!(&Ustring::from_vec_with_nul_unchecked(vec!(0)), "");
    }
    assert!(Ustring::from_vec_with_nul(vec!(b'1')).is_err());
    assert_eq!(
        Ustring::from_vec_with_nul(vec!(b'1', 0))
            .unwrap()
            .into_vec(),
        vec!(b'1')
    );
    assert_eq!(
        Ustring::from_vec_with_nul(vec!())
            .unwrap()
            .into_vec_with_nul(),
        vec!(0)
    );

    assert_eq!(&Ustring::from_string("".to_string()), "");
    assert_eq!(&Ustring::from_string_with_nul("".to_string()).unwrap(), "");
    assert_eq!(
        &Ustring::from_string_with_nul("1\0".to_string()).unwrap(),
        "1"
    );
    assert!(Ustring::from_string_with_nul("1".to_string()).is_err());
    assert_eq!(
        Ustring::from_string_with_nul("1\0".to_string())
            .unwrap()
            .into_string()
            .unwrap(),
        "1".to_string()
    );
    assert_eq!(
        Ustring::from_string_with_nul("1\0".to_string())
            .unwrap()
            .into_string_with_nul()
            .unwrap(),
        "1\0".to_string()
    );
    assert!(Bstr::from_bytes(b"\xff")
        .to_ustring()
        .into_string()
        .is_err());
    assert!(Bstr::from_bytes(b"\xff")
        .to_ustring()
        .into_string_with_nul()
        .is_err());

    assert_eq!(&Ustring::from_c_string(CString::new("abc").unwrap()), "abc");
    assert_eq!(
        ustr!("abc").to_ustring().into_c_string().unwrap().as_ustr(),
        "abc"
    );
    assert!(ustr!("a\0bc").to_ustring().into_c_string().is_err());

    assert_eq!(
        &Ustring::from_os_string(OsStr::new("abc").to_os_string()),
        "abc"
    );
    assert!(&Ustring::from_os_string_with_nul(OsStr::new("abc").to_os_string()).is_err());
    assert_eq!(
        &Ustring::from_os_string_with_nul(OsStr::new("abc\0").to_os_string()).unwrap(),
        "abc"
    );
    assert_eq!(ustr!("abc").to_ustring().into_os_string(), "abc");
    assert_eq!(ustr!("abc").to_ustring().into_os_string_with_nul(), "abc\0");

    assert_eq!(
        &Ustring::from_path_buf(Path::new("abc").to_path_buf()),
        "abc"
    );
    assert!(&Ustring::from_path_buf_with_nul(Path::new("abc").to_path_buf()).is_err());
    assert_eq!(
        &Ustring::from_path_buf_with_nul(Path::new("abc\0").to_path_buf()).unwrap(),
        "abc"
    );
    assert_eq!(ustr!("abc").to_ustring().into_path_buf().as_os_str(), "abc");
    assert_eq!(
        ustr!("abc")
            .to_ustring()
            .into_path_buf_with_nul()
            .as_os_str(),
        "abc\0"
    );

    assert_eq!(ustr!("abc").to_ustring().as_ustr_mut(), "abc");

    assert_eq!(Ustring::new(), Ustring::default());

    let us = ustr!("abc").to_ustring();
    {
        let b: &Bstr = us.borrow();
        assert_eq!(b, "abc");
    }

    {
        let b: &Ustr = us.borrow();
        assert_eq!(b, "abc");
    }

    assert_eq!(&from::<Ustring, _>(vec!(b'a')), "a");
    assert_eq!(&from::<Ustring, _>("a".to_string()), "a");
    assert_eq!(&from::<Ustring, _>(OsStr::new("a").to_os_string()), "a");
    assert_eq!(&from::<Ustring, _>(Path::new("a").to_path_buf()), "a");
    assert_eq!(&from::<Ustring, _>(CString::new("a").unwrap()), "a");
    assert_eq!(&from::<Vec<u8>, _>(ustr!("a").to_ustring()), b"a");
    assert_eq!(&from::<OsString, _>(ustr!("a").to_ustring()), "a");
    assert_eq!(from::<PathBuf, _>(ustr!("a").to_ustring()).as_os_str(), "a");

    assert_eq!(am::<[u8], _>(&mut format_ustr!("abc")), ustr!("abc"));
    assert_eq!(ar::<[u8], _>(&format_ustr!("abc")), ustr!("abc"));
    assert_eq!(ar::<OsStr, _>(&format_ustr!("abc")), ustr!("abc"));
    assert_eq!(ar::<Path, _>(&format_ustr!("abc")), ustr!("abc"));

    assert_eq!(format!("{:?}", format_ustr!("abc")), "\"abc\"");

    assert_eq!(&**format_ustr!("abc"), "abc");
    assert_eq!(&mut **format_ustr!("abc"), "abc");

    assert_eq!(
        try_from::<String, _>(format_ustr!("a")).unwrap(),
        format!("a")
    );
    assert!(
        try_from::<String, _>(Ustr::from_bytes(b"\xff\0").unwrap().to_ustring()).is_err()
    );

    assert_eq!(
        try_from::<CString, _>(format_ustr!("a")).unwrap(),
        CString::new("a").unwrap()
    );
    assert!(try_from::<CString, _>(format_ustr!("\0")).is_err());
}

#[test]
fn ustrptr() {
    let mut buf = UstrPtr::default();

    buf.push("abc");
    buf.extend(["def"].iter().copied());

    unsafe {
        assert_eq!(CStr::from_ptr(*buf.as_ptr()).as_ustr(), "abc");
        assert_eq!(
            CStr::from_ptr(*(buf.as_ptr() as *const *const c::c_char).add(1)).as_ustr(),
            "def"
        );
        assert!((*(buf.as_ptr() as *const *const c::c_char).add(2)).is_null());
    }

    let buf: UstrPtr = ["abc"].iter().copied().collect();

    unsafe {
        assert_eq!(CStr::from_ptr(*buf.as_ptr()).as_ustr(), "abc");
        assert!((*(buf.as_ptr() as *const *const c::c_char).add(1)).is_null());
    }
}

#[test]
fn read() {
    let vec = format!("abc").into_bytes();
    let mut buf = &vec[..];

    assert_eq!(&UapiReadExt::read_to_new_ustring(&mut buf).unwrap(), "abc");

    let mut buf = &vec[..];

    let mut us = Ustring::new();
    assert_eq!(UapiReadExt::read_to_ustring(&mut buf, &mut us).unwrap(), 3);
    assert_eq!(&us, "abc");
}

#[test]
fn into() {
    assert_eq!(&"a".into_ustr().to_ustring(), "a");
    assert_eq!(&"a".to_string().into_ustr().to_ustring(), "a");
    assert_eq!(&"a".as_bytes().into_ustr().to_ustring(), "a");
    assert_eq!(&"a".to_string().into_bytes().into_ustr().to_ustring(), "a");
    assert_eq!(&ustr!("a").as_bstr().into_ustr().to_ustring(), "a");
    assert_eq!(&ustr!("a").into_ustr().to_ustring(), "a");
    assert_eq!(&ustr!("a").to_ustring().into_ustr().to_ustring(), "a");
    assert_eq!(&OsStr::new("a").into_ustr().to_ustring(), "a");
    assert_eq!(
        &OsStr::new("a").to_os_string().into_ustr().to_ustring(),
        "a"
    );
    assert_eq!(&Path::new("a").into_ustr().to_ustring(), "a");
    assert_eq!(&Path::new("a").to_path_buf().into_ustr().to_ustring(), "a");
    assert_eq!(
        &CStr::from_bytes_with_nul(b"a\0")
            .unwrap()
            .into_ustr()
            .to_ustring(),
        "a"
    );
    assert_eq!(
        &CStr::from_bytes_with_nul(b"a\0")
            .unwrap()
            .to_owned()
            .into_ustr()
            .to_ustring(),
        "a"
    );
    assert_eq!(&(&"a".into_ustr()).into_ustr().to_ustring(), "a");
    assert_eq!(&"a".into_ustr().into_ustr().to_ustring(), "a");
}

#[test]
fn eq() {
    macro_rules! c {
        ($e:expr) => {{
            assert_eq!($e, "a".as_bytes());
            assert_eq!("a".as_bytes(), $e);
            assert_eq!($e, "a");
            assert_eq!("a", $e);
            assert_eq!($e, CStr::from_bytes_with_nul(b"a\0").unwrap());
            assert_eq!(CStr::from_bytes_with_nul(b"a\0").unwrap(), $e);
            assert_eq!($e, OsStr::new("a"));
            assert_eq!(OsStr::new("a"), $e);
            assert_eq!($e, Path::new("a"));
            assert_eq!(Path::new("a"), $e);
            assert_eq!($e, &"a".to_string().into_bytes());
            assert_eq!(&"a".to_string().into_bytes(), $e);
            assert_eq!($e, &"a".to_string());
            assert_eq!(&"a".to_string(), $e);
            assert_eq!($e, &CStr::from_bytes_with_nul(b"a\0").unwrap().to_owned());
            assert_eq!(&CStr::from_bytes_with_nul(b"a\0").unwrap().to_owned(), $e);
            assert_eq!($e, &OsStr::new("a").to_owned());
            assert_eq!(&OsStr::new("a").to_owned(), $e);
            assert_eq!($e, &Path::new("a").to_owned());
            assert_eq!(&Path::new("a").to_owned(), $e);
            assert_eq!($e, ustr!("a"));
            assert_eq!(ustr!("a"), $e);
            assert_eq!($e, ustr!("a").as_bstr());
            assert_eq!(ustr!("a").as_bstr(), $e);
            assert_eq!($e, &ustr!("a").to_ustring());
            assert_eq!(&ustr!("a").to_ustring(), $e);
        }};
    }

    c!(ustr!("a"));
    c!(ustr!("a").as_bstr());
    c!(&ustr!("a").to_owned());
}

#[test]
fn bytes() {
    assert_eq!(ustr!("a").as_bstr().bytes(), b"a");
    assert_eq!(ustr!("a").into_ustr().bytes(), b"a");
    assert_eq!(Bytes::bytes("a"), b"a");
    assert_eq!("a".as_bytes().bytes(), b"a");
    assert_eq!(OsStr::new("a").bytes(), b"a");
    assert_eq!(Path::new("a").bytes(), b"a");
    assert_eq!(CStr::from_bytes_with_nul(b"a\0").unwrap().bytes(), b"a");
}

#[test]
fn as_ustr() {
    assert_eq!(ustr!("a").as_ustr(), "a");
    assert_eq!(AsUstr::as_ustr(&ustr!("a").to_ustring()), "a");
    assert_eq!(CStr::from_bytes_with_nul(b"a\0").unwrap().as_ustr(), "a");
    assert_eq!(
        CStr::from_bytes_with_nul(b"a\0")
            .unwrap()
            .to_owned()
            .as_ustr(),
        "a"
    );
}
