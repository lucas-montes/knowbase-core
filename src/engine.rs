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

use wide::f64x4;

#[derive(Debug)]
struct GzipClassifierResult {
    label: usize,
    result: f64,
}

impl GzipClassifierResult {
    fn new(first: &[u8], record: &StringRecord) -> Self {
        let (text, label) = get_text_label(record);
        let result = ncd(first, &text);
        Self { label, result }
    }
    fn sim(first: &[u8], record: &[StringRecord]) -> Vec<Self> {
        let (text, label) = get_text_label(&record[0]);
        let result = ncd(first, &text);
        vec![Self { label, result }]
    }
}

impl Ord for GzipClassifierResult {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.result.partial_cmp(&other.result).unwrap()
    }
}

impl PartialEq for GzipClassifierResult {
    fn eq(&self, other: &Self) -> bool {
        (self.result == other.result) & (self.label == other.label)
    }
}

impl Eq for GzipClassifierResult {}
impl PartialOrd for GzipClassifierResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.result.partial_cmp(&other.result)
    }
}

fn get_text_label(record: &StringRecord) -> (Vec<u8>, usize) {
    let text = get_text(record).as_bytes().to_vec();
    let label: usize = record.get(0).expect("no class index").parse().unwrap();
    (text, label)
}

pub fn compress(text: &[u8]) -> Vec<u8> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::fast());
    let _ = e.write_all(text);
    e.finish().unwrap()
}

fn calculate(line: &StringRecord, lines: &Vec<StringRecord>) -> Vec<GzipClassifierResult> {
    let (first, label) = get_text_label(line);
    lines
        .par_chunks(4)
        .flat_map(|l| GzipClassifierResult::sim(&first, &l))
        .collect::<Vec<GzipClassifierResult>>()
}

fn get_text(record: &StringRecord) -> String {
    let mut text = record.get(1).expect("no title").to_owned();
    text.push_str(" ");
    text.push_str(record.get(2).expect("no descripton"));
    return text;
}

fn ncd(first: &[u8], line: &[u8]) -> f64 {
    let first_comp = compress(first).len();
    let second_compress = compress(&line).len();
    let both_str = [first, "".as_bytes(), line].concat();
    let both = compress(&both_str).len() as f64;
    let m = min(first_comp, second_compress) as f64;
    let ma = max(first_comp, second_compress) as f64;
    (both - m) / ma
}

pub async fn prueba(label: String, item: String, training: PathBuf) {
    let records = get_records(training);
    let (a, b) = item.split_at(item.len() / 2);
    let test = StringRecord::from(vec![label, a.to_string(), b.to_string()]);
    println!("first calculation");
    let mut sorted_distances = calculate(&test, &records);
//    sorted_distances.dedup();
    sorted_distances.par_sort();
    let (result, _) = sorted_distances.split_at(10); 
    println!("{:?}", result);
    let mut f = [0,0,0,0];
    result.iter().for_each(
        |r| match r.label {
            1 => f[0] += 1,
            2 => f[1] += 1,
            3 => f[2] += 1,
            _ => f[3] += 1,
        }
    );
    println!("{:?}", f);
}

pub async fn process(label: String, item: String, training: PathBuf) {
    let records = get_records(training);
    let (a, b) = item.split_at(item.len() / 2);
    let test = StringRecord::from(vec![label, a.to_string(), b.to_string()]);
    println!("first calculation");
    let mut sorted_distances = calculate(&test, &records);
//    sorted_distances.dedup();
    sorted_distances.par_sort();
    let (result, _) = sorted_distances.split_at(10); 
    println!("{:?}", result);
    let mut f = [0,0,0,0];
    result.iter().for_each(
        |r| match r.label {
            1 => f[0] += 1,
            2 => f[1] += 1,
            3 => f[2] += 1,
            _ => f[3] += 1,
        }
    );
    println!("{:?}", f);
}

fn get_records(filepath: PathBuf) -> Vec<StringRecord> {
    csv::Reader::from_path(filepath)
        .expect("something went wrong reading the csv")
        .into_records()
        .map(|r| r.unwrap())
        .collect()
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
