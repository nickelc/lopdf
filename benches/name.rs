use criterion::{black_box, criterion_group, criterion_main, Criterion};

use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_while1};
use nom::combinator::{map, map_opt};
use nom::multi::{fold_many1, many0};
use nom::sequence::preceded;

type Result<'a, O> = nom::IResult<&'a [u8], O>;

#[allow(dead_code)]
mod hex_char;

use hex_char::hex_char_take3_verify_match as hex_char;

#[inline]
fn is_whitespace(c: u8) -> bool {
    b" \t\n\r\0\x0C".contains(&c)
}

#[inline]
fn is_delimiter(c: u8) -> bool {
    b"()<>[]{}/%".contains(&c)
}

#[inline]
fn is_regular(c: u8) -> bool {
    !is_whitespace(c) && !is_delimiter(c)
}

#[derive(Debug)]
enum NameFragment<'a> {
    HexChar(u8),
    String(&'a [u8]),
}

fn name_fragment(input: &[u8]) -> Result<NameFragment> {
    let string = take_while1(|c| c != b'#' && is_regular(c));
    alt((
        map(hex_char, NameFragment::HexChar),
        map(string, NameFragment::String),
    ))(input)
}

fn name_fold_many1_fragments(input: &[u8]) -> Result<Vec<u8>> {
    let name = fold_many1(name_fragment, Vec::new(), |mut buf, fragment| {
        match fragment {
            NameFragment::HexChar(c) => buf.push(c),
            NameFragment::String(s) => buf.extend_from_slice(s),
        }
        buf
    });
    preceded(tag("/"), name)(input)
}

fn name_many0_chars(input: &[u8]) -> Result<Vec<u8>> {
    let chars = alt((
        hex_char,
        map_opt(take(1usize), |c: &[u8]| {
            if c[0] != b'#' && is_regular(c[0]) {
                Some(c[0])
            } else {
                None
            }
        }),
    ));
    preceded(tag("/"), many0(chars))(input)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("name_fold_many1_fragments", |b| {
        b.iter(|| name_fold_many1_fragments(black_box(b"/Name1#cc")));
    });
    c.bench_function("name_many0_chars", |b| {
        b.iter(|| name_many0_chars(black_box(b"/Name1#cc")));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
