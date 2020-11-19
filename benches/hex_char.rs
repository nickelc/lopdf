use criterion::{black_box, criterion_group, criterion_main, Criterion};

use nom::bytes::complete::{tag, take, take_while_m_n};
use nom::character::is_hex_digit;
use nom::combinator::{map_res, verify};
use nom::sequence::preceded;

type Result<'a, O> = nom::IResult<&'a [u8], O>;

fn hex_char_take_while_m_n(input: &[u8]) -> Result<u8> {
    let parser = map_res(take_while_m_n(2, 2, is_hex_digit), |x| {
        u8::from_str_radix(std::str::from_utf8(x).unwrap(), 16)
    });
    preceded(tag("#"), parser)(input)
}

fn hex_char_take2_verify_iter(input: &[u8]) -> Result<u8> {
    let check = |h: &[u8]| {
        h.iter().copied().all(is_hex_digit)
    };
    let parser = map_res(
        verify(take(2usize), check),
        |x| u8::from_str_radix(std::str::from_utf8(x).unwrap(), 16),
    );
    preceded(tag("#"), parser)(input)
}

fn hex_char_take2_verify_iter_std(input: &[u8]) -> Result<u8> {
    let check = |h: &[u8]| {
        h.iter().all(u8::is_ascii_hexdigit)
    };
    let parser = map_res(
        verify(take(2usize), check),
        |x| u8::from_str_radix(std::str::from_utf8(x).unwrap(), 16),
    );
    preceded(tag("#"), parser)(input)
}

fn hex_char_take2_verify_slice(input: &[u8]) -> Result<u8> {
    let check = |h: &[u8]| {
        is_hex_digit(h[0]) && is_hex_digit(h[1])
    };
    let parser = map_res(
        verify(take(2usize), check),
        |x| u8::from_str_radix(std::str::from_utf8(x).unwrap(), 16),
    );
    preceded(tag("#"), parser)(input)
}

fn hex_char_take2_verify_slice_std(input: &[u8]) -> Result<u8> {
    let check = |h: &[u8]| {
        h[0].is_ascii_hexdigit() && h[1].is_ascii_hexdigit()
    };
    let parser = map_res(
        verify(take(2usize), check),
        |x| u8::from_str_radix(std::str::from_utf8(x).unwrap(), 16),
    );
    preceded(tag("#"), parser)(input)
}

fn hex_char_take3_verify_match(input: &[u8]) -> Result<u8> {
    let check = |b: &[u8]| match b {
        [b'#', a, b] if a.is_ascii_hexdigit() && b.is_ascii_hexdigit() => true,
        _ => false,
    };
    let mut parser = map_res(verify(take(3usize), check), |x: &[u8]| {
        u8::from_str_radix(std::str::from_utf8(&x[1..]).unwrap(), 16)
    });
    parser(input)
}

fn hex_char_take3_verify_unsafe(input: &[u8]) -> Result<u8> {
    let check = |b: &[u8]| unsafe {
        b.get_unchecked(0) == &b'#'
            && b.get_unchecked(1).is_ascii_hexdigit()
            && b.get_unchecked(2).is_ascii_hexdigit()
    };
    let mut parser = map_res(verify(take(3usize), check), |x: &[u8]| {
        u8::from_str_radix(std::str::from_utf8(&x[1..]).unwrap(), 16)
    });
    parser(input)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("hex_char_take_while_m_n", |b| {
        b.iter(|| hex_char_take_while_m_n(black_box(b"#65")));
    });
    c.bench_function("hex_char_take2_verify_iter", |b| {
        b.iter(|| hex_char_take2_verify_iter(black_box(b"#65")));
    });
    c.bench_function("hex_char_take2_verify_iter_std", |b| {
        b.iter(|| hex_char_take2_verify_iter_std(black_box(b"#65")));
    });
    c.bench_function("hex_char_take2_verify_slice", |b| {
        b.iter(|| hex_char_take2_verify_slice(black_box(b"#65")));
    });
    c.bench_function("hex_char_take2_verify_slice_std", |b| {
        b.iter(|| hex_char_take2_verify_slice_std(black_box(b"#65")));
    });
    c.bench_function("hex_char_take3_verify_match", |b| {
        b.iter(|| hex_char_take3_verify_match(black_box(b"#65")));
    });
    c.bench_function("hex_char_take3_verify_unsafe", |b| {
        b.iter(|| hex_char_take3_verify_unsafe(black_box(b"#65")));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
