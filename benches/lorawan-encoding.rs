use criterion::{criterion_group, criterion_main, Criterion};
use lorawan_encoding::crypto::soft::SoftCrypto;
use lorawan_encoding::phy_payload::mac_payload::uplink::Uplink;
use lorawan_encoding::types::{AppSKey, NwkSKey};
use std::alloc::System;
use std::sync::atomic::{AtomicUsize, Ordering};
use zerocopy::{FromBytes as _, TryFromBytes};

extern crate std;

#[global_allocator]
static GLOBAL: trallocator::Trallocator<System> = trallocator::Trallocator::new(System);

fn bench_complete_data_payload_fhdr(c: &mut Criterion) {
    let cnt = AtomicUsize::new(0);
    GLOBAL.usage();
    c.bench_function("data_payload_headers_parsing", |b| {
        b.iter(|| {
            cnt.fetch_add(1usize, Ordering::SeqCst);
            let data = data_payload();

            if let Ok(payload) = Uplink::try_ref_from_bytes(&data) {
                assert_eq!(payload.f_cnt(), 1u16);

                let fctrl = payload.f_ctrl();

                assert_eq!(fctrl.f_opts_len(), 0);

                assert!(!fctrl.f_pending(), "no f_pending");

                assert!(!fctrl.ack(), "no ack");

                assert!(fctrl.adr(), "ADR");
            } else {
                panic!("failed to parse DataPayload");
            }
        })
    });
    let n = cnt.load(Ordering::SeqCst);
    println!(
        "Approximate memory usage per iteration: {} from {}",
        GLOBAL.usage() / n,
        n
    );
}

fn bench_complete_data_payload_mic_validation(c: &mut Criterion) {
    let nwk_s_key = NwkSKey::read_from_bytes(&[2; 16]).unwrap();
    let app_s_key = AppSKey::read_from_bytes(&[1; 16]).unwrap();
    let mut crypto = SoftCrypto::new(nwk_s_key, app_s_key);
    let cnt = AtomicUsize::new(0);
    GLOBAL.usage();
    c.bench_function("data_payload_mic_validation", |b| {
        b.iter(|| {
            cnt.fetch_add(1usize, Ordering::SeqCst);
            let data = data_payload();
            if let Ok(payload) = Uplink::try_ref_from_bytes(&data) {
                assert!(payload.validate_mic(&mut crypto, 1, 18));
            } else {
                panic!("failed to parse DataPayload");
            }
        })
    });
    let n = cnt.load(Ordering::SeqCst);
    println!(
        "Approximate memory usage per iteration: {} from {}",
        GLOBAL.usage() / n,
        n
    );
}

fn bench_complete_data_payload_decrypt(c: &mut Criterion) {
    let nwk_s_key = NwkSKey::read_from_bytes(&[2; 16]).unwrap();
    let app_s_key = AppSKey::read_from_bytes(&[1; 16]).unwrap();
    let mut crypto = SoftCrypto::new(nwk_s_key, app_s_key);
    let cnt = AtomicUsize::new(0);
    GLOBAL.usage();
    c.bench_function("data_payload_decrypt", |b| {
        b.iter(|| {
            cnt.fetch_add(1usize, Ordering::SeqCst);
            let mut data = data_payload();

            if let Ok(mac_payload) = Uplink::try_mut_from_bytes(&mut data) {
                mac_payload.encrypt(&mut crypto, 1, 5);
                assert_eq!(&mac_payload.frm_payload().data[..5], b"hello");
            } else {
                panic!("failed to parse DataPayload");
            }
        })
    });
    let n = cnt.load(Ordering::SeqCst);
    println!(
        "Approximate memory usage per iteration: {} from {}",
        GLOBAL.usage() / n,
        n
    );
}

criterion_group!(
    benches,
    bench_complete_data_payload_fhdr,
    bench_complete_data_payload_mic_validation,
    bench_complete_data_payload_decrypt
);
criterion_main!(benches);

fn data_payload() -> [u8; 18] {
    [
        0x40, 0x04, 0x03, 0x02, 0x01, 0x80, 0x01, 0x00, 0x01, 0xa6, 0x94, 0x64, 0x26, 0x15, 0xd6,
        0xc3, 0xb5, 0x82,
    ]
}
