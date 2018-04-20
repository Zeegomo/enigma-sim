#[macro_use]
extern crate measure_time;
#[macro_use]
extern crate log;

use std::io::{self, BufRead};
use std::collections::*;
use std::fs::File;
use std::io::prelude::*;

static ROT1: [u8; 27] = [4, 10, 12, 5, 11, 6, 3, 16, 21, 25, 13, 19, 14, 22, 24, 7, 23, 20, 18, 15, 0, 8, 1, 17, 2, 9, 16];
static ROT2: [u8; 27] = [0, 9, 3, 10, 18, 8, 17, 20, 23, 1, 11, 7, 22, 19, 12, 2, 16, 6, 25, 13, 15, 24, 5, 21, 14, 4, 4];
static ROT3: [u8; 27] = [1, 3, 5, 7, 9, 11, 2, 15, 17, 19, 23, 21, 25, 13, 24, 4, 8, 22, 6, 0, 10, 12, 20, 18, 16, 14, 21];

static ALPHABET: [u8; 26] = ['A' as u8, 'B' as u8, 'C' as u8, 'D' as u8, 'E' as u8, 'F' as u8, 'G' as u8, 'H' as u8, 'I' as u8, 'J' as u8, 'K' as u8, 'L' as u8, 'M' as u8, 'N' as u8, 'O' as u8, 'P' as u8, 'Q' as u8, 'R' as u8, 'S' as u8, 'T' as u8, 'U' as u8, 'V' as u8, 'W' as u8, 'X' as u8, 'Y' as u8, 'Z' as u8];
static REFLECTORN: [u8; 26] = [24, 17, 20, 7, 16, 18, 11, 3, 15, 23, 13, 6, 14, 10, 12, 8, 4, 1, 5, 25, 2, 22, 21, 9, 0, 19];


fn main() {
    let stdin = io::stdin();

    let mut prob: Vec<String> = Vec::new();
    let mut value: Vec<u64> = Vec::new();
    let mut english_quad_prob: HashMap<Vec<u8>, u64> = HashMap::new();
    init_quadgram(&mut english_quad_prob);
    //init_quad_vect(&mut prob, &mut value);

    println!("Number of letters to encrypt:");
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let num = u64::from_str_radix(&line, 10).unwrap(); //TODO handle exceptions

    println!("Insert rotor order:");
    let mut line = stdin.lock().lines().next().unwrap().unwrap();
    let numrot1 = u64::from_str_radix(&line, 10).unwrap() - 1; //TODO handle exceptions
    line = stdin.lock().lines().next().unwrap().unwrap();
    let numrot2 = u64::from_str_radix(&line, 10).unwrap() - 1; //TODO handle exceptions
    line = stdin.lock().lines().next().unwrap().unwrap();
    let numrot3 = u64::from_str_radix(&line, 10).unwrap() - 1; //TODO handle exceptions

    println!("Insert rotor settings");
    let mut seq_inizio_rotori = [0 as u8; 3];
    line = stdin.lock().lines().next().unwrap().unwrap();
    seq_inizio_rotori[0] = line.into_bytes()[0] - 65;
    line = stdin.lock().lines().next().unwrap().unwrap();
    seq_inizio_rotori[1] = line.into_bytes()[0] - 65;
    line = stdin.lock().lines().next().unwrap().unwrap();
    seq_inizio_rotori[2] = line.into_bytes()[0] - 65;

    println!("Enigma correctly set!");
    println!("Insert text to encrypt");
    line = stdin.lock().lines().next().unwrap().unwrap();
    let mut da_cifrare = line.into_bytes();
    for i in 0..da_cifrare.len() {
        da_cifrare[i] = da_cifrare[i] - 65;
    }
    println!("{:?}", da_cifrare);
    println!("{:?}", seq_inizio_rotori);
    println!("{}", da_cifrare[0]);
    let mut encrypted = enigma_encrypt(&da_cifrare, num, numrot1, numrot2, numrot3, &seq_inizio_rotori);
    print!("ciphertext: ");
    for i in 0..encrypted.len() {
        print!("{}", encrypted[i] as char);
    }
    println!();
    let mut score = 0;
    for i in 0..da_cifrare.len() - 3 {
        score += quadgramma(&english_quad_prob, &da_cifrare[i..i + 4].to_vec());
    }
    println!("plaintext score: {}", score);

    println!("Cracking the encryption (i accidentally forgot the key)");
    print_time!("decryption function");
    let mut newCifrato = vec![0 as u8; num as usize];
    let mut newCifratoMax = vec![0 as u8; num as usize];
    let mut quadDaCercare = vec![0 as u8; num as usize];
    let mut rot1Max = 0;
    let mut rot2Max = 0;
    let mut rot3Max = 0;
    let mut probMax = 0;
    let mut letteraRotSet1Max = 0;
    let mut letteraRotSet2Max = 0;
    let mut letteraRotSet3Max = 0;
    for r1 in 0..3 {
        for r2 in 0..3 {
            for r3 in 0..3 {
                if r2 != r1 && r3 != r1 && r3 != r2 {
                    for letteraRotSet1 in 0..26 {
                        for letteraRotSet2 in 0..26 {
                            for letteraRotSet3 in 0..26 {
                                seq_inizio_rotori[0] = letteraRotSet1;
                                seq_inizio_rotori[1] = letteraRotSet2;
                                seq_inizio_rotori[2] = letteraRotSet3;
                                let mut score: u64 = 0;
                                quadDaCercare = encrypted.to_vec();
                                for i in 0..quadDaCercare.len() {
                                    quadDaCercare[i] = quadDaCercare[i] - 65;
                                }
                                let mut new_cifrato = enigma_encrypt(&quadDaCercare, num, r1, r2, r3, &seq_inizio_rotori);
                                for i in 0..new_cifrato.len() {
                                    new_cifrato[i] = new_cifrato[i] - 65;
                                }
                                for i in 0..num as usize -3{
                                    //score = score + quad_vect(&prob, &value, &new_cifrato[i..i+4].to_vec());
                                    score = score + quadgramma(&english_quad_prob, &new_cifrato[i..i+4].to_vec());
                                }
                                if score > probMax {
                                    rot1Max = r1;
                                    rot2Max = r2;
                                    rot3Max = r3;
                                    letteraRotSet1Max = letteraRotSet1;
                                    letteraRotSet2Max = letteraRotSet2;
                                    letteraRotSet3Max = letteraRotSet3;
                                    probMax = score;
                                    for qqq in 0..num {
                                        newCifratoMax[qqq as usize] = new_cifrato[qqq as usize];
                                    }
                                    println!("Round:{:?}, {}, {}, {}, {}", newCifratoMax, probMax, letteraRotSet1Max, letteraRotSet2Max, letteraRotSet3Max);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Final:{:?}, {}", newCifratoMax, probMax);
    trace_time!(format!("{:?}", "yep"));
    for i in 0..encrypted.len() {
        print!("{}", (newCifratoMax[i] + 65) as char);
    }
    println!();
}

fn init_quadgram(hash_map: &mut HashMap<Vec<u8>, u64>) {
    println!("Letter frequency initialization...");
    let lines = lines_from_file("/home/zeegomo/Documents/robe/english_quadgrams.txt");
    let mut line;
    let mut current;
    for i in 0..lines.len() {
        line = &lines[i];
        current = line.split_whitespace();
        let string = current.next().unwrap().to_owned();
        let value = u64::from_str_radix(current.next().unwrap(), 10).unwrap().to_owned();
        //println!("{}, {}", string, value);
        let mut string = string.into_bytes();
        for i in 0..string.len() {
            string[i] = string[i] - 65;
        }
        hash_map.insert(string, value);
    }

}

fn init_quad_vect(prob: &mut Vec<String>, value: &mut Vec<u64>){
    println!("Letter frequency initialization...");
    let lines = lines_from_file("/home/zeegomo/Documents/robe/english_quadgrams.txt");
    let mut line;
    let mut current;
    for i in 0..lines.len() {
        line = &lines[i];
        current = line.split_whitespace();
        let string = current.next().unwrap().to_owned();
        let value_t = u64::from_str_radix(current.next().unwrap(), 10).unwrap().to_owned();
        //println!("{}, {}", string, value);
        prob.push(string);
        value.push(value_t);
    }
}

pub fn lines_from_file(filename: &str) -> Vec<String> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let lines: Vec<String> = file_contents.split("\n")
        .map(|s: &str| s.to_string())
        .collect();
    lines
}

fn enigma_encrypt(message: &[u8], length: u64, numrot1: u64, numrot2: u64, numrot3: u64, rotor_setting_input_ref: &[u8]) -> Vec<u8> {
    //println!("Encrypting...");
    let mut numrot = [numrot1, numrot2, numrot3];
    numrot.sort_unstable();

    let mut messaggioCifrato = vec![0 as u8; length as usize];
    let mut enigma = message.to_vec();
    //let stdin = io::stdin();
    let mut rotor_setting = [[0 as u8; 3]; 27];
    /*let mut lines = lines_from_file("/home/zeegomo/Documents/robe/input.txt");

    for i in 0..3 {
        let mut iter = lines[i].split_whitespace();
        for z in 0..27 as usize {
            rotor_setting[z][numrot[i] as usize] = u8::from_str_radix(iter.next().unwrap(), 10).unwrap().to_owned();
        }
    }*/
    for z in 0..27{
        rotor_setting[z][numrot[0] as usize] = ROT1[z];
    }
    for z in 0..27{
        rotor_setting[z][numrot[1] as usize] = ROT2[z];
    }
    for z in 0..27{
        rotor_setting[z][numrot[2] as usize] = ROT3[z];
    }

    let mut rotor_setting_input = rotor_setting_input_ref.to_vec();
    for ii in 0..length as usize {
        rotor_setting_input[2] = rotor_setting_input[2] + 1;
        rotor_setting_input[2] = rotor_setting_input[2] % 26;
        if rotor_setting_input[2] == (rotor_setting[26][numrot3 as usize] + 1) {
            rotor_setting_input[1] = rotor_setting_input[1] + 1;
            rotor_setting_input[1] = rotor_setting_input[1] % 26;
        }
        if rotor_setting_input[1] == rotor_setting[26][numrot2 as usize] {
            rotor_setting_input[1] = rotor_setting_input[1] + 1;
            rotor_setting_input[1] = rotor_setting_input[1] % 26;
        }
        if rotor_setting_input[1] == (rotor_setting[26][numrot2 as usize] + 1) {
            rotor_setting_input[0] = rotor_setting_input[0] + 1;
            rotor_setting_input[0] = rotor_setting_input[0] % 26;
        }

        //tutte le volte che entra una lettera il rotore scatta avanti di 1
        enigma[ii] = ((((enigma[ii] as i16 + rotor_setting_input[2] as i16) % 26) + 26) % 26) as u8;
        enigma[ii] = ((enigma[ii] % 26) + 26) % 26;
        enigma[ii] = rotor_setting[enigma[ii] as usize][numrot3 as usize];
        enigma[ii] = ((((enigma[ii] as i16 - rotor_setting_input[2] as i16 + rotor_setting_input[1] as i16) % 26) + 26) % 26) as u8;
        //enigma[ii] = ((enigma[ii] % 26) + 26) % 26;
        enigma[ii] = rotor_setting[enigma[ii] as usize][numrot2 as usize];
        enigma[ii] = ((((enigma[ii] as i16 - rotor_setting_input[1] as i16 + rotor_setting_input[0] as i16) % 26) + 26) % 26) as u8;
        //enigma[ii] = ((enigma[ii] % 26) + 26) % 26;
        enigma[ii] = rotor_setting[enigma[ii] as usize][numrot1 as usize];
        enigma[ii] = ((((enigma[ii] as i16 - rotor_setting_input[0] as i16) % 26) + 26) % 26) as u8;
        //enigma[ii] = ((enigma[ii] % 26) + 26) % 26;
        enigma[ii] = REFLECTORN[enigma[ii] as usize];
        //ritorno
        enigma[ii] = ((((enigma[ii] as i16 + rotor_setting_input[0] as i16) % 26) + 26) % 26) as u8;
        //enigma[ii] = ((enigma[ii] % 26) + 26) % 26;
        for kk in 0..26 as usize {
            if enigma[ii] == rotor_setting[kk][numrot1 as usize] {
                enigma[ii] = kk as u8;
                break;
            }
        }

        enigma[ii] = ((((enigma[ii] as i16 - rotor_setting_input[0] as i16 + rotor_setting_input[1] as i16) % 26) + 26) % 26) as u8;
        //enigma[ii] = ((enigma[ii] % 26) + 26) % 26;
        for kk in 0..26 as usize {
            if enigma[ii] == rotor_setting[kk][numrot2 as usize] {
                enigma[ii] = kk as u8;
                break;
            }
        }

        enigma[ii] = ((((enigma[ii] as i16 - rotor_setting_input[1] as i16 + rotor_setting_input[2] as i16) % 26) + 26) % 26) as u8;
        //enigma[ii] = ((enigma[ii] % 26) + 26) % 26;
        for kk in 0..26 as usize {
            if enigma[ii] == rotor_setting[kk][numrot3 as usize] {
                enigma[ii] = kk as u8;
                break;
            }
        }
        enigma[ii] = ((((enigma[ii] as i16 - rotor_setting_input[2] as i16) % 26) + 26) % 26) as u8;
        enigma[ii] = ((enigma[ii] % 26) + 26) % 26;
    }
    for ii in 0..length as usize {
        messaggioCifrato[ii] = ALPHABET[enigma[ii] as usize];
    }
    messaggioCifrato
}

fn quadgramma(hash_map: &HashMap<Vec<u8>, u64>, mut quad: &Vec<u8>) -> u64 {
    match hash_map.get(quad) {
        Some(value) => return value.clone(),
        None => return 0,
    }
}

fn quad_vect(prob: &Vec<String>, value: &Vec<u64>, quad: &Vec<u8>) -> u64{
    let mut ret = 0;
    let mut quad = quad.clone();
    for i in 0..quad.len(){
        quad[i] = quad[i] + 65;
    }
    let string = String::from_utf8_lossy(&quad);
    for i in 0..389373{
        if string == prob[i]{
            ret = value[i];
            break;
        }
    }
    ret
}