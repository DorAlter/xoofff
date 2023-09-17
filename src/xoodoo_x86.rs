/// Maximum number of rounds one can request to have when applying Xoodoo\[n_r\] permutation i.e. n_r <= MAX_ROUNDS
///
/// See table 2 of https://ia.cr/2018/767
const MAX_ROUNDS: usize = 12;

/// Xoodoo\[n_r\] round constants, taken from table 2 of https://ia.cr/2018/767
const RC: [u32; MAX_ROUNDS] = [
    0x00000058, 0x00000038, 0x000003c0, 0x000000d0, 0x00000120, 0x00000014, 0x00000060, 0x0000002c,
    0x00000380, 0x000000f0, 0x000001a0, 0x00000012,
];

use core::arch::x86_64::*;

use super::{Xoodoo, ROUND_KEYS};


pub fn permute<const ROUNDS: usize>(st: &mut [u32]) {
    debug_assert!(
        st.len() == 12,
        "Xoodoo permutation state must have 12 lanes !"
    );
    debug_assert!(
        ROUNDS <= MAX_ROUNDS,
        "Requested rounds must be < MAX_ROUNDS !"
    );
    unsafe {
        let rho_east_2 = _mm_set_epi32(0x0605_0407, 0x0201_0003, 0x0e0d_0c0f, 0x0a09_080b);
        let mut a = _mm_loadu_si128(st.as_ptr().add(0 * 4) as *const _);
        let mut b = _mm_loadu_si128(st.as_ptr().add(4 * 4) as *const _);
        let mut c = _mm_loadu_si128(st.as_ptr().add(8 * 4) as *const _);
        let start = MAX_ROUNDS - ROUNDS;
        for ridx in start..MAX_ROUNDS {
            round_key = RC[ridx];
            let mut p = _mm_shuffle_epi32(_mm_xor_si128(_mm_xor_si128(a, b), c), 0x93);
            let mut e = _mm_or_si128(_mm_slli_epi32(p, 5), _mm_srli_epi32(p, 32 - 5));
            p = _mm_or_si128(_mm_slli_epi32(p, 14), _mm_srli_epi32(p, 32 - 14));
            e = _mm_xor_si128(e, p);
            a = _mm_xor_si128(a, e);
            b = _mm_xor_si128(b, e);
            c = _mm_xor_si128(c, e);
            b = _mm_shuffle_epi32(b, 0x93);
            c = _mm_or_si128(_mm_slli_epi32(c, 11), _mm_srli_epi32(c, 32 - 11));
            a = _mm_xor_si128(a, _mm_set_epi32(0, 0, 0, round_key as _));
            a = _mm_xor_si128(a, _mm_andnot_si128(b, c));
            b = _mm_xor_si128(b, _mm_andnot_si128(c, a));
            c = _mm_xor_si128(c, _mm_andnot_si128(a, b));
            b = _mm_or_si128(_mm_slli_epi32(b, 1), _mm_srli_epi32(b, 32 - 1));
            c = _mm_shuffle_epi8(c, rho_east_2);
        }
        _mm_storeu_si128(st.as_mut_ptr().add(0 * 4) as *mut _, a);
        _mm_storeu_si128(st.as_mut_ptr().add(4 * 4) as *mut _, b);
        _mm_storeu_si128(st.as_mut_ptr().add(8 * 4) as *mut _, c);
    }
}


