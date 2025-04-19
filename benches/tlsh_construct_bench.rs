use crate::data_row::{prepare_data, DataRow};
use divan::{bench, black_box, Bencher};
use tlsh_orig::{BucketKind, ChecksumKind};
use tlsh_orig::Version::Version4;

mod data_row;

static TRUE: [u8; 1] = [1];
static FALSE: [u8; 1] = [0];

fn main() {
    // create_sample_data();

    // let data = prepare_data();
    //
    // // For debugging
    // for d in data.iter().take(100) {
    //     println!("{}", d.id);
    //     println!("{:?}", std::str::from_utf8(&hash_tlsh2(d)).unwrap());
    //     println!("{:?}", hash_fast_tlsh(d).to_string());
    //     println!("{:?}", hash_simbiota_tlsh(d));
    //     println!("{:?}", hash_tlsh_orig(d));
    // }

    divan::main();
}

fn hash_tlsh2(data_row: &DataRow) -> [u8; 72] {
    use tlsh2::TlshBuilder128_1;

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

    builder.build().unwrap().hash()
}

#[bench]
fn construct_tlsh2(bencher: Bencher) {
    let data = prepare_data();

    bencher.bench(|| {
        let mut hashes: Vec<[u8; 72]> = Vec::with_capacity(data.len());

        black_box({
            for data_row in &data {
                hashes.push(hash_tlsh2(data_row));
            }
        });
    });
}

fn hash_fast_tlsh(data_row: &DataRow) -> tlsh::Tlsh {
    use tlsh::GeneratorType;
    use tlsh::TlshGenerator;

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

    builder.finalize().unwrap()
}

#[bench]
fn construct_fast_tlsh(bencher: Bencher) {
    use tlsh::Tlsh;

    let data = prepare_data();

    bencher.bench(|| {
        let mut hashes: Vec<Tlsh> = Vec::with_capacity(data.len());

        black_box({
            for data_row in &data {
                hashes.push(hash_fast_tlsh(data_row));
            }
        });
    });
}

fn hash_simbiota_tlsh(data_row: &DataRow) -> String {
    use simbiota_tlsh::TLSHBuilder;

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

    builder.get_hash().unwrap().to_digest()
}

#[bench]
fn construct_simbiota_tlsh(bencher: Bencher) {
    let data = prepare_data();

    bencher.bench(|| {
        let mut hashes: Vec<String> = Vec::with_capacity(data.len());

        black_box({
            for data_row in &data {
                hashes.push(hash_simbiota_tlsh(data_row));
            }
        });
    });
}

fn hash_tlsh_orig(data_row: &DataRow) -> String {
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

    builder.build().unwrap().hash()
}

#[bench]
fn construct_tlsh_orig(bencher: Bencher) {
    let data = prepare_data();

    bencher.bench(|| {
        let mut hashes: Vec<String> = Vec::with_capacity(data.len());

        black_box({
            for data_row in &data {
                hashes.push(hash_tlsh_orig(data_row));
            }
        });
    });
}