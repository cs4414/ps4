pub fn range(lo: uint, hi: uint, it: |uint|) {
    let mut iter = lo;
    while iter < hi {
        it(iter);
        iter += 1;
    }
}

#[inline]
pub fn to_str_bytes(num: int, radix: int, f: |u8|) {
    let neg = num < 0;
    let mut deccum = if neg { -num } else { num };

    // Radix can be as low as 2, so we need 64 characters for a number
    // near 2^64, plus another one for a possible '-' character.
    let mut buf = [0u8, ..65];
    let mut cur = 0;

    loop {
        let digit = deccum % radix;
        buf[cur] = match digit as u8 {
            i @ 0..9 => '0' as u8 + i,
            i        => 'a' as u8 + (i-10),
        };
        cur += 1;
        deccum /= radix as int;
        if deccum == 0 { break; }
    }

    if neg {
        f('-' as u8);
    }

    while cur > 0 {
        cur -= 1;
        f(buf[cur]);
    }
}
