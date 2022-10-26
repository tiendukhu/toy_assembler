use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct IdSequence {
    id: String,
    sequence: String,
}

impl IdSequence {
    fn new(id: String, sequence: String) -> IdSequence {
        IdSequence { id: id, sequence: sequence }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file = File::open(args[1].to_string()).unwrap();
    let reader = BufReader::new(file);
    let mut id = String::from("");
    let mut storage: Vec<IdSequence> = Vec::new();
    for line in reader.lines() {
        let l = line.as_ref().unwrap();
        let s = l.split("");
        let vec: Vec<&str> = s.collect();        
        let mut name = String::from("");
        if vec[1] == ">" {
            for i in 1 .. vec.len() {
                if vec[i] == " " {
                    name = vec[2 .. i].join("");
                    break;
                } else if vec[i] != "" { 
                    name = vec[2 .. vec.len()].join("");
                }
            }
            id = name;
        } else {
            storage.push(IdSequence::new(id.to_string(), l.to_string()));
        }
    }
    let mut sequences: Vec<String> = Vec::new();
    for i in storage {
        sequences.push(i.sequence);
    }
    let mut sequences: Vec<&str> = sequences.iter().map(|s| &**s).collect();
    sequences.sort();
    let result = assemble(sequences[0], sequences[1 .. ].to_vec());
    let mut file = File::create("assembly.txt")?;
    file.write(result.as_bytes())?;
    Ok(())
}

fn score(sequence1: &str, sequence2: &str, offset: isize) -> usize {
    let sequence1_length = sequence1.len() as isize;
    let sequence2_length = sequence2.len() as isize;
    let start_of_overlap = *[0 - offset, 0]
        .iter()
        .max()
        .unwrap();
    let end_of_overlap = *[sequence2_length - offset, sequence2_length, sequence1_length - offset]
        .iter()
        .min()
        .unwrap();
    let mut total_score = 0;
    let sequence1: Vec<&str> = sequence1.split("").collect();
    let sequence2: Vec<&str> = sequence2.split("").collect();
    for i1 in start_of_overlap .. end_of_overlap + 1 {
        let i2 = (i1 + offset) as usize;
        let i1 = i1 as usize;
        if sequence2[i1] == sequence1[i2] {
            total_score += 1;
        }
    }
    return total_score;
}

fn find_best_offset(sequence1: &str, sequence2: &str) -> (usize, isize, String, String) {
    let lowest_offset: isize = 1 - (sequence2.len() as isize);
    let highest_offset: isize = sequence1.len() as isize;
    let mut tuples: Vec<(usize, isize, String, String)> = Vec::new();
    for i in lowest_offset .. highest_offset {
        tuples.push((score(sequence1, sequence2, i), i, sequence2.to_string(), sequence1.to_string()));
    }
    tuples.sort_by_key(|k| k.0);
    let result = &tuples[tuples.len() - 1];
    return result.to_owned();
}

fn find_best_match(sequence1: &str, others: Vec<&str>) -> (usize, isize, String, String) {
    let mut tuples: Vec<(usize, isize, String, String)> = Vec::new();
    for sequence2 in others {
        if sequence2 != sequence1 {
            tuples.push(find_best_offset(sequence1, sequence2))
        }
    }
    tuples.sort_by_key(|k| k.0);
    let result = &tuples[tuples.len() - 1];
    return result.to_owned();
}

fn consensus((score, offset, sequence1, sequence2): (usize, isize, String, String)) -> String {
    let sequence1_length = sequence1.len() as isize;
    let left_overhang: usize = *[0, offset]
        .iter()
        .max()
        .unwrap() as usize;
    let right_overhang: usize = (sequence1_length + offset) as usize;
    let sequence2_length = sequence2.len() as isize;
    let mut sequence2_right_overhang = String::from("");
    let mut sequence2_left_overhang = String::from("");
    if left_overhang != 0 {
        sequence2_left_overhang = sequence2[0 .. left_overhang].to_string();
    }
    if right_overhang < sequence2.len() {
        sequence2_right_overhang = sequence2[right_overhang .. ].to_string();
    }
    return sequence2_left_overhang + &sequence1 + &sequence2_right_overhang;
}

fn assemble(sequence1: &str, mut others: Vec<&str>) -> String {
    let best_matching_other = find_best_match(sequence1, others.to_vec());
    let consensus_sequence = consensus(best_matching_other.clone());
    if others.len() == 1 {
        return consensus_sequence
    } else {
        let best_matching_sequence = best_matching_other.2;
        let index = others.iter().position(|x| *x == best_matching_sequence).unwrap();
        others.remove(index);
        return assemble(&consensus_sequence, others);
    }
}
