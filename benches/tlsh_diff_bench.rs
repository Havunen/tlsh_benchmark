use crate::data_row::prepare_data;
use divan::{bench, black_box, Bencher};
use simbiota_tlsh::{TLSHBuilder, TLSH};
use tlsh::FuzzyHashType;
use tlsh2::TlshBuilder128_1;
use tlsh_orig::{BucketKind, ChecksumKind};
use tlsh_orig::Version::Version4;

mod data_row;


static TRUE: [u8; 1] = [1];
static FALSE: [u8; 1] = [0];

const NUM_OF_ROWS: usize = 1000;

fn main() {
    divan::main();
}

#[bench]
fn diff_tlsh2(bencher: Bencher) {
    let data = prepare_data();
    let mut hashes = Vec::with_capacity(data.len());
    for data_row in data.iter().take(NUM_OF_ROWS) {
        let mut builder = TlshBuilder128_1::new();

        builder.update(&data_row.id.to_ne_bytes());
        builder.update(&data_row.name.as_bytes());
        builder.update(&data_row.value.as_bytes());
        builder.update(&data_row.d1.to_ne_bytes());
        builder.update(&data_row.d2.to_ne_bytes());
        builder.update(&data_row.d3.to_ne_bytes());
        builder.update(&data_row.d4.to_ne_bytes());
        builder.update(&data_row.d5.to_ne_bytes());
        builder.update(&data_row.d6.to_ne_bytes());
        builder.update(&data_row.d7.to_ne_bytes());
        builder.update(&data_row.d8.to_ne_bytes());

        builder.update(&data_row.f1.to_ne_bytes());
        builder.update(&data_row.f2.to_ne_bytes());
        builder.update(&data_row.f3.to_ne_bytes());
        builder.update(&data_row.f4.to_ne_bytes());

        builder.update(if data_row.b1 { &TRUE } else { &FALSE });
        builder.update(if data_row.b2 { &TRUE } else { &FALSE });
        builder.update(if data_row.b3 { &TRUE } else { &FALSE });
        builder.update(if data_row.b4 { &TRUE } else { &FALSE });

        builder.update(&data_row.s1.as_bytes());
        builder.update(&data_row.s2.as_bytes());
        
        hashes.push(builder.build().unwrap());
    }

    bencher.bench(|| {
        let mut results: Vec<i32> = Vec::with_capacity(hashes.len() * hashes.len());
        
        black_box({
            for j in 0..hashes.len() {
                let hash = &hashes[j];
                for i in 0..hashes.len() {
                    results.push(hash.diff(&hashes[i], true));
                }
            }
        });
    });
}

#[bench]
fn diff_fast_tlsh(bencher: Bencher) {
    use tlsh::GeneratorType;
    use tlsh::TlshGenerator;
    
    let data = prepare_data();
    let mut hashes = Vec::with_capacity(data.len());
    for data_row in data.iter().take(NUM_OF_ROWS) {
        let mut builder = TlshGenerator::new();

        builder.update(&data_row.id.to_ne_bytes());
        builder.update(data_row.name.as_bytes());
        builder.update(data_row.value.as_bytes());
        builder.update(&data_row.d1.to_ne_bytes());
        builder.update(&data_row.d2.to_ne_bytes());
        builder.update(&data_row.d3.to_ne_bytes());
        builder.update(&data_row.d4.to_ne_bytes());
        builder.update(&data_row.d5.to_ne_bytes());
        builder.update(&data_row.d6.to_ne_bytes());
        builder.update(&data_row.d7.to_ne_bytes());
        builder.update(&data_row.d8.to_ne_bytes());

        builder.update(&data_row.f1.to_ne_bytes());
        builder.update(&data_row.f2.to_ne_bytes());
        builder.update(&data_row.f3.to_ne_bytes());
        builder.update(&data_row.f4.to_ne_bytes());

        builder.update(if data_row.b1 { &TRUE } else { &FALSE });
        builder.update(if data_row.b2 { &TRUE } else { &FALSE });
        builder.update(if data_row.b3 { &TRUE } else { &FALSE });
        builder.update(if data_row.b4 { &TRUE } else { &FALSE });

        builder.update(data_row.s1.as_bytes());
        builder.update(data_row.s2.as_bytes());

        hashes.push(builder.finalize().unwrap());
    }

    bencher.bench(|| {
        let mut results: Vec<u32> = Vec::with_capacity(hashes.len() * hashes.len());

        black_box({
            for j in 0..hashes.len() {
                let hash = &hashes[j];
                for i in 0..hashes.len() {
                    results.push(hash.compare(&hashes[i]));
                }
            }
        });
    });
}

#[bench]
fn diff_simbiota_tlsh(bencher: Bencher) {

    let data = prepare_data();
    let mut hashes = Vec::with_capacity(data.len());
    for data_row in data.iter().take(NUM_OF_ROWS) {
        let mut builder = TLSHBuilder::new();

        builder.update(&data_row.id.to_ne_bytes());
        builder.update(data_row.name.as_bytes());
        builder.update(data_row.value.as_bytes());
        builder.update(&data_row.d1.to_ne_bytes());
        builder.update(&data_row.d2.to_ne_bytes());
        builder.update(&data_row.d3.to_ne_bytes());
        builder.update(&data_row.d4.to_ne_bytes());
        builder.update(&data_row.d5.to_ne_bytes());
        builder.update(&data_row.d6.to_ne_bytes());
        builder.update(&data_row.d7.to_ne_bytes());
        builder.update(&data_row.d8.to_ne_bytes());

        builder.update(&data_row.f1.to_ne_bytes());
        builder.update(&data_row.f2.to_ne_bytes());
        builder.update(&data_row.f3.to_ne_bytes());
        builder.update(&data_row.f4.to_ne_bytes());

        builder.update(if data_row.b1 { &TRUE } else { &FALSE });
        builder.update(if data_row.b2 { &TRUE } else { &FALSE });
        builder.update(if data_row.b3 { &TRUE } else { &FALSE });
        builder.update(if data_row.b4 { &TRUE } else { &FALSE });

        builder.update(data_row.s1.as_bytes());
        builder.update(data_row.s2.as_bytes());

        builder.finalize();

        hashes.push(builder.get_hash().unwrap());
    }

    bencher.bench(|| {
        let mut results: Vec<i32> = Vec::with_capacity(hashes.len() * hashes.len());

        black_box({
            for j in 0..hashes.len() {
                let hash = &hashes[j];
                for i in 0..hashes.len() {
                    results.push(TLSH::diff(hash, &hashes[i]));
                }
            }
        });
    });
}

#[bench]
fn diff_tlsh_orig(bencher: Bencher) {

    let data = prepare_data();
    let mut hashes = Vec::with_capacity(data.len());
    for data_row in data.iter().take(NUM_OF_ROWS) {
        let mut builder = tlsh_orig::TlshBuilder::new(BucketKind::Bucket128, ChecksumKind::OneByte, Version4);

        builder.update(&data_row.id.to_ne_bytes());
        builder.update(data_row.name.as_bytes());
        builder.update(data_row.value.as_bytes());
        builder.update(&data_row.d1.to_ne_bytes());
        builder.update(&data_row.d2.to_ne_bytes());
        builder.update(&data_row.d3.to_ne_bytes());
        builder.update(&data_row.d4.to_ne_bytes());
        builder.update(&data_row.d5.to_ne_bytes());
        builder.update(&data_row.d6.to_ne_bytes());
        builder.update(&data_row.d7.to_ne_bytes());
        builder.update(&data_row.d8.to_ne_bytes());

        builder.update(&data_row.f1.to_ne_bytes());
        builder.update(&data_row.f2.to_ne_bytes());
        builder.update(&data_row.f3.to_ne_bytes());
        builder.update(&data_row.f4.to_ne_bytes());

        builder.update(if data_row.b1 { &TRUE } else { &FALSE });
        builder.update(if data_row.b2 { &TRUE } else { &FALSE });
        builder.update(if data_row.b3 { &TRUE } else { &FALSE });
        builder.update(if data_row.b4 { &TRUE } else { &FALSE });

        builder.update(data_row.s1.as_bytes());
        builder.update(data_row.s2.as_bytes());

        hashes.push(builder.build().unwrap());
    }

    bencher.bench(|| {
        let mut results: Vec<usize> = Vec::with_capacity(hashes.len() * hashes.len());

        black_box({
            for j in 0..hashes.len() {
                let hash = &hashes[j];
                for i in 0..hashes.len() {
                    results.push(hash.diff(&hashes[i], true));
                }
            }
        });
    });
}