use divan::{bench, black_box, Bencher};
use tlsh::{GeneratorType, Tlsh};
use crate::data_row::DataRow;

mod data_row;


static TRUE: [u8; 1] = [1];
static FALSE: [u8; 1] = [0];

fn main() {
    // create_sample_data();
    //
    // let data = prepare_data();
    //
    // for d in &data {
    //     println!("{}", d.id);
    //     println!("{:?}", hash_tlsh2(d));
    //     println!("{:?}", hash_fast_tlsh(d));
    // }

    divan::main();
}

fn prepare_data() -> Vec<DataRow> {
    let file = std::fs::File::open("benches/data_rows.json").expect("Failed to open data_rows.json");
    let data_rows: Vec<DataRow> = serde_json::from_reader(file).expect("Failed to deserialize data rows");
    data_rows
}

fn create_sample_data() {
    let data_rows: Vec<DataRow> = (0..10000).map(|id| DataRow::new(id)).collect();

    let file = std::fs::File::create("benches/data_rows.json").expect("Failed to create data_rows.json");
    serde_json::to_writer(file, &data_rows).expect("Failed to serialize data rows");

    println!("Created {} sample data rows", data_rows.len());
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
fn build_tlsh2(bencher: Bencher) {
    let data = prepare_data();

    bencher.bench_local(|| {
        let mut hashes: Vec<[u8; 72]> = Vec::with_capacity(data.len());

        black_box({
            for data_row in &data {
                hashes.push(hash_tlsh2(data_row));
            }
        });
    });
}

fn hash_fast_tlsh(data_row: &DataRow) -> Tlsh {
    use tlsh::{TlshGenerator};

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
fn build_fast_tlsh(bencher: Bencher) {
    let data = prepare_data();

    bencher.bench_local(|| {
        let mut hashes: Vec<Tlsh> = Vec::with_capacity(data.len());

        black_box({
            for data_row in &data {
                hashes.push(hash_fast_tlsh(data_row));
            }
        });
    });
}

#[test]
pub fn run_all_once() {
    hash_tlsh2(&DataRow::new(0));
    hash_fast_tlsh(&DataRow::new(0));
}