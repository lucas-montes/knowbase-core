use crate::file_handler::get_word_count;
use crate::models::{File, FileWordRelation, Manager, Word};
use csv::StringRecord;
use flate2::{write::ZlibEncoder, Compression};
use rayon::prelude::*;
use rec_rsys::{algorithms::knn::KNN, models::Item, similarity::SimilarityAlgos, utils::argsort};
use std::{
    cmp::{max, min},
    fs,
    io::prelude::*,
    path::PathBuf,
};

struct GzipClassifierText {
    label: u32,
    compressed: Vec<u8>,
    encoded: Vec<u8>,
    size: f32,
}

impl GzipClassifierText {
    pub fn from_record(record: &StringRecord) ->  GzipClassifierText {
        let (text, label) = Self::get_text_label(record);
        let compressed = compress(&text);
        let size = compressed.len() as f32;
        GzipClassifierText {
            label: label,
            compressed: compressed,
            encoded: text,
            size: size,
        }
    }

    fn get_text_label(record: &StringRecord) -> (Vec<u8>, u32) {
        let text = get_text(record).as_bytes().to_vec();
        let label: u32 = record.get(0).expect("no class index").parse().unwrap();
        (text, label)
    }
}

pub fn compress(text: &[u8]) -> Vec<u8> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::fast());
    let _ = e.write_all(text);
    e.finish().unwrap()
}
pub fn less_is_more(records: &Vec<StringRecord>) -> Vec<Item> {
    println!("less is more start");
    records.par_iter().map(|r| calculate(r, records)).collect()
}

fn get_records(filepath: PathBuf) -> Vec<StringRecord> {
    csv::Reader::from_path(filepath)
        .expect("something went wrong reading the csv")
        .into_records()
        .map(|r| r.unwrap())
        .collect()
}

fn calculate(line: &StringRecord, lines: &Vec<StringRecord>) -> Item {
    let (first, label) = GzipClassifierText::get_text_label(line);
    let values: Vec<f32> = lines
        .par_iter()
        .map(|l| ncd(&first, get_text(&l).as_bytes()))
        .collect();
    Item::new(label, argsort(&values), None)
}

fn get_text(record: &StringRecord) -> String {
    let mut text = record.get(1).expect("no title").to_owned();
    text.push_str(" ");
    text.push_str(record.get(2).expect("no descripton"));
    text
}

fn ncd(first: &[u8], line: &[u8]) -> f32 {
    let first_comp = compress(first).len();
    let second_compress = compress(&line).len();
    let both_str = [first, "".as_bytes(), line].concat();
    let both = compress(&both_str).len();
    ((both - min(first_comp, second_compress)) / max(first_comp, second_compress)) as f32
}

pub async fn process(label: String, item: String, training: PathBuf) {
    let records = get_records(training);
    let (a, b) = item.split_at(item.len() / 2);
    let test = StringRecord::from(vec![label, a.to_string(), b.to_string()]);
    println!("first calculation");
    let result = KNN::new(calculate(&test, &records), less_is_more(&records), 10)
        .result(SimilarityAlgos::Cosine);
    println!("{:?}", result);
}

async fn tf(paths: Vec<PathBuf>) {
    for filepath in paths {
        let word_count = get_word_count(&filepath);
        let fullpath = fs::canonicalize(filepath).unwrap();
        let file = File::new(fullpath.to_string_lossy().to_string())
            .get_or_create()
            .await;

        for (key, value) in word_count.iter() {
            let word = Word::new(key.to_string()).get_or_create().await;
            let _ = FileWordRelation::new(word.id, file.id, *value)
                .create_or_update()
                .await;
        }
    }
}
