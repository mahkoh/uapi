extern crate proc_macro;

use lazy_static::lazy_static;
use proc_macro2::{Punct, TokenStream};
use quote::quote;
use regex::Regex;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input, Attribute, Ident, LitInt, LitStr, Path,
};

lazy_static! {
    static ref TC: TestConditions = {
        let mut tc = TestConditions {
            root: unsafe { libc::geteuid() == 0 },
            ..Default::default()
        };

        let mut utsname = unsafe { std::mem::zeroed() };
        unsafe {
            libc::uname(&mut utsname);
        }

        let regex = regex::bytes::Regex::new(r"^([0-9]+)\.([0-9]+)").unwrap();

        if let Some(m) = regex.captures(unsafe {
            &*(&utsname.release[..] as *const [libc::c_char] as *const [u8])
        }) {
            let parse = |i| {
                std::str::from_utf8(m.get(i).unwrap().as_bytes())
                    .unwrap()
                    .parse()
                    .unwrap()
            };
            let major: i32 = parse(1);
            let minor: i32 = parse(2);

            tc.linux_4_16 = major > 4 || (major == 4 && minor >= 16);
            tc.linux_5_2 = major > 5 || (major == 5 && minor >= 2);
            tc.linux_5_6 = major > 5 || (major == 5 && minor >= 6);
            tc.linux_5_9 = major > 5 || (major == 5 && minor >= 9);
        }

        tc
    };
}

#[derive(Default, Debug)]
struct TestConditions {
    root: bool,
    linux_4_16: bool,
    linux_5_2: bool,
    linux_5_6: bool,
    linux_5_9: bool,
}

impl Parse for TestConditions {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut tc = TestConditions::default();

        while !input.is_empty() {
            let name = input.parse::<Ident>()?;
            match &*name.to_string() {
                "root" => tc.root = true,
                "linux_4_16" => tc.linux_4_16 = true,
                "linux_5_2" => tc.linux_5_2 = true,
                "linux_5_6" => tc.linux_5_6 = true,
                "linux_5_9" => tc.linux_5_9 = true,
                n => {
                    return Err(syn::Error::new(
                        name.span(),
                        format!("unknown test condition {}", n),
                    ));
                }
            }
            if !input.is_empty() {
                parse_comma(input)?;
            }
        }

        Ok(tc)
    }
}

#[proc_macro_attribute]
pub fn test_if(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let tc = parse_macro_input!(attr as TestConditions);
    let ignore = (tc.root && !TC.root)
        || (tc.linux_4_16 && !TC.linux_4_16)
        || (tc.linux_5_2 && !TC.linux_5_2)
        || (tc.linux_5_6 && !TC.linux_5_6)
        || (tc.linux_5_9 && !TC.linux_5_9);
    #[allow(clippy::match_bool)] // already disabled upstream
    let ignore = match ignore {
        false => quote!(),
        true => quote!(#[ignore]),
    };
    let item = TokenStream::from(item);
    quote!(
        #[test]
        #ignore
        #item
    )
    .into()
}

#[proc_macro_attribute]
pub fn notest(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = TokenStream::from(item);
    quote!(
        #[deprecated = "there are no tests for this api"]
        #item
    )
    .into()
}

#[proc_macro_attribute]
pub fn beta(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = TokenStream::from(item);
    quote!(
        #[deprecated = "beta features are subject to change at any time"]
        #item
    )
    .into()
}

struct Man(String);

impl Parse for Man {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        lazy_static! {
            static ref MAN_REG: Regex =
                regex::Regex::new(r"([a-zA-Z]*)\((([0-9]+)([^)]*))\)").unwrap();
        }

        if input.peek(Ident) {
            let name = input.parse::<Ident>()?;
            let content;
            parenthesized!(content in input);
            let section = content.parse::<LitInt>()?;
            let doc = format!(
                "[`{0}({1})`](http://man7.org/linux/man-pages/man{1}/{0}.{1}.html)",
                name, section
            );
            return Ok(Man(doc));
        }
        let doc = input.parse::<LitStr>()?.value();
        let res = MAN_REG.replace_all(
            &doc,
            "[`$1($2)`](http://man7.org/linux/man-pages/man$3/$1.$2.html)",
        );
        Ok(Man(res.to_string()))
    }
}

#[proc_macro_attribute]
pub fn man(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let man = parse_macro_input!(attr as Man).0;
    let item = TokenStream::from(item);
    quote!(
        #[doc = #man]
        #item
    )
    .into()
}

struct SockaddrRequest {
    attr: Vec<Attribute>,
    get: bool,
    set: bool,
    level: Ident,
    optname: Ident,
    strct: Path,
}

fn parse_punct(input: ParseStream, chr: char) -> syn::Result<()> {
    let punct = input.parse::<Punct>()?;
    match punct.as_char() {
        c if c == chr => Ok(()),
        c => Err(syn::Error::new(
            punct.span(),
            format!("Expected '{}', got '{}'", chr, c),
        )),
    }
}

fn parse_comma(input: ParseStream) -> syn::Result<()> {
    parse_punct(input, ',')
}

fn parse_equals(input: ParseStream) -> syn::Result<()> {
    parse_punct(input, '=')
}

impl Parse for SockaddrRequest {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attr = syn::Attribute::parse_outer(input)?;
        let directions = input.parse::<Ident>()?;
        parse_comma(input)?;
        let level = input.parse::<Ident>()?;
        parse_comma(input)?;
        let optname = input.parse::<Ident>()?;
        let mut strct = syn::parse_str("c::c_int").unwrap();
        while !input.is_empty() {
            parse_comma(input)?;
            let name = input.parse::<Ident>()?;
            parse_equals(input)?;
            match &*name.to_string() {
                "ty" => {
                    strct = input.parse::<syn::Path>()?;
                }
                n => {
                    return Err(syn::Error::new(
                        name.span(),
                        format!("unknown named parameter {}", n),
                    ));
                }
            }
        }
        let (get, set) = match &*directions.to_string() {
            "get" => (true, false),
            "set" => (false, true),
            "bi" => (true, true),
            o => {
                return Err(syn::Error::new(
                    directions.span(),
                    format!("Expected get/set/bi, got {}", o),
                ))
            }
        };
        Ok(Self {
            attr,
            get,
            set,
            level,
            optname,
            strct,
        })
    }
}

#[proc_macro]
pub fn sock_opt(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let SockaddrRequest {
        attr,
        get,
        set,
        level,
        optname,
        strct,
    } = parse_macro_input!(item as SockaddrRequest);
    let mut ts = TokenStream::new();
    if get {
        let man = format!(
            "getsockopt(2) with level = `{}` and optname = `{}`",
            level.to_string(),
            optname.to_string()
        );
        let fnname = Ident::new(
            &format!("getsockopt_{}", optname.to_string().to_lowercase()),
            optname.span(),
        );
        ts.extend(quote!(
            #[man(#man)]
            #(#attr)*
            pub fn #fnname(sockfd: c::c_int) -> Result<#strct> {
                let mut val: #strct = unsafe { mem::zeroed() };
                let mut len = mem::size_of::<#strct>() as _;
                let res = unsafe { c::getsockopt(sockfd, c::#level, c::#optname, &mut val as *mut _ as *mut _, &mut len) };
                map_err!(res).map(|_| val)
            }
        ));
    }
    if set {
        let man = format!(
            "setsockopt(2) with level = `{}` and optname = `{}`",
            level.to_string(),
            optname.to_string()
        );
        let fnname = Ident::new(
            &format!("setsockopt_{}", optname.to_string().to_lowercase()),
            optname.span(),
        );
        ts.extend(quote!(
            #[man(#man)]
            #(#attr)*
            pub fn #fnname(sockfd: c::c_int, mut val: #strct) -> Result<()> {
                let mut len = mem::size_of::<#strct>() as _;
                let res = unsafe { c::setsockopt(sockfd, c::#level, c::#optname, &val as *const _ as *const _, len) };
                map_err!(res).map(drop)
            }
        ));
    }
    ts.into()
}
