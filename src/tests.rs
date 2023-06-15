use crate::Xoofff;
use rand::{thread_rng, RngCore};
use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use test_case::test_case;

/// Test functional correctness of Xoofff deck function, by using
/// known answer tests, generated by following steps described on
/// https://gist.github.com/itzmeanjan/504113021dec30a0909e5f5b47a5bde5
#[test]
fn test_xoofff_kat() {
    let kat_file = "./kats/Xoofff_KAT.txt";
    let file = File::open(kat_file).unwrap();
    let mut reader = BufReader::new(file).lines();

    loop {
        if let Some(line) = reader.next() {
            // key to be used for instantiating deck function
            let key = line.unwrap();
            let key = key.split(" ").collect::<Vec<_>>()[1];
            let key = hex::decode(key).unwrap();

            // message to be absorbed into deck function
            let msg = reader.next().unwrap().unwrap();
            let msg = msg.split(" ").collect::<Vec<_>>()[1];
            let msg = hex::decode(msg).unwrap();

            // # -of bytes to be skipped before message squeezing begins
            let q = reader.next().unwrap().unwrap();
            let q = q.split(" ").collect::<Vec<_>>()[1]
                .parse::<usize>()
                .unwrap();

            let out = reader.next().unwrap().unwrap();
            let out = out.split(" ").collect::<Vec<_>>()[1];

            // expected squeezed bytes
            let expected = hex::decode(out).unwrap();
            // to be squeezed bytes
            let mut computed = vec![0u8; expected.len()];

            let mut deck = Xoofff::new(&key);
            deck.absorb(&msg);
            deck.finalize(0, 0, q);
            deck.squeeze(&mut computed);

            assert_eq!(
                expected,
                computed,
                "key = {}, msg = {}, q = {}",
                hex::encode(&key),
                hex::encode(&msg),
                q
            );

            reader.next().unwrap().unwrap(); // skip the empty line
        } else {
            // no more test vectors, time to break out of loop
            break;
        }
    }
}

#[test_case(32, 0, 32, 0b1, 1, 0; "key = 32B message = 0B digest = 32B offset = 0B")]
#[test_case(16, 32, 64, 0b11, 2, 0; "key = 16B message = 32B digest = 64B offset = 0B")]
#[test_case(32, 64, 128, 0b101, 3, 1; "key = 32B message = 64B digest = 128B offset = 1B")]
#[test_case(32, 128, 256, 0b101, 3, 2; "key = 32B message = 128B digest = 256B offset = 2B")]
#[test_case(32, 256, 512, 0b1101, 4, 4; "key = 32B message = 256B digest = 512B offset = 4B")]
#[test_case(32, 512, 1024, 0b10101, 5, 8; "key = 32B message = 512B digest = 1024B offset = 8B")]
#[test_case(32, 1024, 2048, 0, 0, 16; "key = 32B message = 1024B digest = 2048B offset = 16B")]
#[test_case(47, 2048, 4096, 0b1, 2, 16; "key = 47B message = 1024B digest = 4096B offset = 16B")]
#[test_case(48, 1024, 32, 0, 0, 32 => panics; "key = 48B message = 1024B digest = 32B offset = 32B")]
#[test_case(24, 1024, 32, 0, 0, 49 => panics; "key = 24B message = 1024B digest = 32B offset = 49B")]
fn test_xoofff_incremental_io(
    klen: usize,
    mlen: usize,
    dlen: usize,
    domain_seperator: u8,
    ds_bit_width: usize,
    offset: usize,
) {
    let mut rng = thread_rng();

    let mut key = vec![0u8; klen];
    let mut msg = vec![0u8; mlen];
    let mut dig0 = vec![0u8; dlen]; // digest from oneshot absorption
    let mut dig1 = vec![0u8; dlen]; // digest from incremental absorption

    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut msg);

    // oneshot absorption
    let mut deck0 = Xoofff::new(&key);
    deck0.absorb(&msg);
    deck0.absorb(&[]); // empty message absorption should have no side effect !
    deck0.finalize(domain_seperator, ds_bit_width, offset);
    deck0.squeeze(&mut dig0);

    // incremental absorption
    let mut deck1 = Xoofff::new(&key);

    let mut off = 0;
    while off < mlen {
        // because we don't want to be stuck in an infinite loop if msg[off] = 0 !
        let elen = cmp::min(cmp::max(msg[off] as usize, 1), mlen - off);

        deck1.absorb(&msg[off..(off + elen)]);
        off += elen;
    }

    deck1.finalize(domain_seperator, ds_bit_width, offset);

    let mut off = 0;
    let mut read = 0u8;
    while off < dlen {
        // because we don't want to be stuck in an infinite loop if read = 0,
        // which is the case, at least in the first iteration.
        let elen = cmp::min(cmp::max(read as usize, 1), dlen - off);

        deck1.squeeze(&mut dig1[off..(off + elen)]);
        off += elen;
        // update how many bytes to squeeze in next iteration ( if any ).
        read = dig1[off - 1];
    }

    assert_eq!(dig0, dig1);
}

#[cfg(feature = "simd")]
#[test]
fn test_xoodoo_simd() {
    use crate::xoodoo::{permute, permutex};
    use core::simd::u32x2;
    use rand::Rng;

    let mut rng = thread_rng();

    let mut state1 = [0u32; 12];
    let mut state2 = [0u32; 12];

    rng.fill(&mut state1);
    rng.fill(&mut state2);

    let mut statex2 = [u32x2::splat(0u32); 12];
    for i in 0..12 {
        statex2[i] = u32x2::from_slice(&[state1[i], state2[i]]);
    }

    permute::<12>(&mut state1);
    permute::<12>(&mut state2);
    permutex::<2, 12>(&mut statex2);

    let mut state12 = [0u32; 12];
    let mut state22 = [0u32; 12];
    for i in 0..12 {
        let [s1, s2] = statex2[i].to_array();
        state12[i] = s1;
        state22[i] = s2;
    }

    assert_eq!(state1, state12);
    assert_eq!(state2, state22);
}
