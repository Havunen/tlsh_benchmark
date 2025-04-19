use fake::Fake;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataRow {
    pub id: usize,

    pub name: String,
    pub value: String,

    pub d1: u8,
    pub d2: u16,
    pub d3: u32,
    pub d4: u64,
    pub d5: i8,
    pub d6: i16,
    pub d7: i32,
    pub d8: i64,


    pub f1: f32,
    pub f2: f64,
    pub f3: f32,
    pub f4: f64,

    pub b1: bool,
    pub b2: bool,
    pub b3: bool,
    pub b4: bool,


    pub s1: String,
    pub s2: String,
}

impl DataRow {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            name: fake::faker::lorem::en::Words(10..11).fake::<Vec<String>>().join(" "),
            value: fake::faker::lorem::en::Words(10..11).fake::<Vec<String>>().join(" "),
            d1: fastrand::u8(0..=255),
            d2: fastrand::u16(0..=65535),
            d3: fastrand::u32(0..=u32::MAX),
            d4: fastrand::u64(0..=u64::MAX),
            d5: fastrand::i8(0..=i8::MAX),
            d6: fastrand::i16(0..=i16::MAX),
            d7: fastrand::i32(0..=i32::MAX),
            d8: fastrand::i64(0..=i64::MAX),
            f1: fastrand::f32(),
            f2: fastrand::f64(),
            f3: fastrand::f32(),
            f4: fastrand::f64(),
            b1: fastrand::bool(),
            b2: fastrand::bool(),
            b3: fastrand::bool(),
            b4: fastrand::bool(),
            s1: fake::faker::lorem::en::Words(10..11).fake::<Vec<String>>().join(" "),
            s2: fake::faker::lorem::en::Words(10..11).fake::<Vec<String>>().join(" "),
        }
    }
}

pub fn prepare_data() -> Vec<DataRow> {
    let file = std::fs::File::open("benches/data_rows.json").expect("Failed to open data_rows.json");
    let data_rows: Vec<DataRow> = serde_json::from_reader(file).expect("Failed to deserialize data rows");
    data_rows
}

#[allow(dead_code)]
pub fn create_sample_data() {
    let data_rows: Vec<DataRow> = (0..10000).map(|id| DataRow::new(id)).collect();

    let file = std::fs::File::create("benches/data_rows.json").expect("Failed to create data_rows.json");
    serde_json::to_writer(file, &data_rows).expect("Failed to serialize data rows");

    println!("Created {} sample data rows", data_rows.len());
}