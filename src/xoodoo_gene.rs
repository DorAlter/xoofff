/// Maximum number of rounds one can request to have when applying Xoodoo\[n_r\] permutation i.e. n_r <= MAX_ROUNDS
///
/// See table 2 of https://ia.cr/2018/767
const MAX_ROUNDS: usize = 12;

/// Xoodoo\[n_r\] round constants, taken from table 2 of https://ia.cr/2018/767
const RC: [u32; MAX_ROUNDS] = [
    0x00000058, 0x00000038, 0x000003c0, 0x000000d0, 0x00000120, 0x00000014, 0x00000060, 0x0000002c,
    0x00000380, 0x000000f0, 0x000001a0, 0x00000012,
];

#[inline(always)]
fn round(st_words: &mut [u32], round_key: u32) {
    let p = [
        st_words[0] ^ st_words[4] ^ st_words[8],
        st_words[1] ^ st_words[5] ^ st_words[9],
        st_words[2] ^ st_words[6] ^ st_words[10],
        st_words[3] ^ st_words[7] ^ st_words[11],
    ];

    let e = [
        p[3].rotate_left(5) ^ p[3].rotate_left(14),
        p[0].rotate_left(5) ^ p[0].rotate_left(14),
        p[1].rotate_left(5) ^ p[1].rotate_left(14),
        p[2].rotate_left(5) ^ p[2].rotate_left(14),
    ];

    let mut tmp = [0u32; 12];

    tmp[0] = e[0] ^ st_words[0] ^ round_key;
    tmp[1] = e[1] ^ st_words[1];
    tmp[2] = e[2] ^ st_words[2];
    tmp[3] = e[3] ^ st_words[3];

    tmp[4] = e[3] ^ st_words[7];
    tmp[5] = e[0] ^ st_words[4];
    tmp[6] = e[1] ^ st_words[5];
    tmp[7] = e[2] ^ st_words[6];

    tmp[8] = (e[0] ^ st_words[8]).rotate_left(11);
    tmp[9] = (e[1] ^ st_words[9]).rotate_left(11);
    tmp[10] = (e[2] ^ st_words[10]).rotate_left(11);
    tmp[11] = (e[3] ^ st_words[11]).rotate_left(11);

    st_words[0] = (!tmp[4] & tmp[8]) ^ tmp[0];
    st_words[1] = (!tmp[5] & tmp[9]) ^ tmp[1];
    st_words[2] = (!tmp[6] & tmp[10]) ^ tmp[2];
    st_words[3] = (!tmp[7] & tmp[11]) ^ tmp[3];

    st_words[4] = ((!tmp[8] & tmp[0]) ^ tmp[4]).rotate_left(1);
    st_words[5] = ((!tmp[9] & tmp[1]) ^ tmp[5]).rotate_left(1);
    st_words[6] = ((!tmp[10] & tmp[2]) ^ tmp[6]).rotate_left(1);
    st_words[7] = ((!tmp[11] & tmp[3]) ^ tmp[7]).rotate_left(1);

    st_words[8] = ((!tmp[2] & tmp[6]) ^ tmp[10]).rotate_left(8);
    st_words[9] = ((!tmp[3] & tmp[7]) ^ tmp[11]).rotate_left(8);
    st_words[10] = ((!tmp[0] & tmp[4]) ^ tmp[8]).rotate_left(8);
    st_words[11] = ((!tmp[1] & tmp[5]) ^ tmp[9]).rotate_left(8);
}

pub fn permute<const ROUNDS: usize>(state: &mut [u32]) {
    debug_assert!(
        state.len() == 12,
        "Xoodoo permutation state must have 12 lanes !"
    );
    debug_assert!(
        ROUNDS <= MAX_ROUNDS,
        "Requested rounds must be < MAX_ROUNDS !"
    );

    let start = MAX_ROUNDS - ROUNDS;
    for ridx in start..MAX_ROUNDS {
        round(state, RC[ridx]);
    }
}

