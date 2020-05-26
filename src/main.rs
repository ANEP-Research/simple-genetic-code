use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter, Write};

use contest_algorithms::scanner::UnsafeScanner;
// Implementation finite deterministic automaton
enum State {
    GetInput(usize),
    Ready,
}

/*
fn is_vowel(idx: u8) -> bool {
    if idx < 14 {
        false
    } else {
        true
    }
}

enum KoreanState {
    Initial,
    Median,
    Final,
}

const KOREAN_TABLE: [char; 27] = [
    'ㄱ', 'ㄴ', 'ㄷ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅅ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ', 'ㅏ', 'ㅐ',
    'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅖ', 'ㅗ', 'ㅛ', 'ㅜ', 'ㅠ', 'ㅡ', 'ㅣ',
];

fn initial_parse(left: u8, right: u8) -> Option<u8> {
    if left == 0 && right == 0 {
        Some(1)
    } else if left == 2 && right == 2 {
        Some(4)
    } else if left == 6 && right == 6 {
        Some(10)
    } else if left == 5 && right == 5 {
        Some(8)
    } else if left == 8 && right == 8 {
        Some(13)
    } else if right == std::u8::MAX {
        Some(match left {
            0 => 0,
            1..3 => left + 1,
            3..6 => left + 2,
            6..7 => left + 3,
            7..9 => left + 4,
            _ => left + 5,
        })
    } else {
        None
    }
}

fn medial_parse(left: u8, right: u8) -> Option<u8> {
    if left == 21 && right == 14 {
        Some(9)
    } else if left == 21 && right == 17 {
        Some(10)
    } else if left == 21 && right == 26 {
        Some(11)
    } else if left == 23 && right == 18 {
        Some(14)
    } else if left == 23 && right == 19 {
        Some(15)
    } else if left == 25 && right == 26 {
        Some(19)
    } else if right == std::u8::MAX {
        let tmp = left - 14;
        Some(match tmp {
            0..9 => tmp,
            9..12 => tmp + 3,
            _ => tmp + 6,
        })
    } else {
        None
    }
}

fn korean(vec: Vec<u8>) -> String {
    let mut res: String = String::new();
    let mut state: KoreanState = KoreanState::Initial;
    let mut tmp: Vec<u8> = Vec::new();
    for word in vec {
        dbg!(KOREAN_TABLE[word as usize]);
        if is_vowel(word) {
            match state {
                KoreanState::Final => {
                    let top = tmp.last().unwrap();
                    res.push_str(
                        &String::from_utf16(&[(((initial_parse(tmp[0], std::u8::MAX).unwrap()
                            as u16)
                            * 588)
                            + (initial_parse(tmp[1], std::u8::MAX).unwrap() as u16) * 28)
                            + 44032])
                        .unwrap(),
                    );
                    tmp = vec![*top, word];
                }
                _ => {
                    tmp.push(word);
                    state = KoreanState::Final;
                }
            }
        } else {
            match state {
                KoreanState::Final => {
                    
                },
                _ => {
                    state = KoreanState::Median;
                    tmp.push(word);
                },
            }         
        }
    }
    res
}
*/

const KOREAN_TABLE: [char; 28] = [
    'ㄱ', 'ㄴ', 'ㄷ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅅ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ', 'ㅏ', 'ㅐ',
    'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ', 'ㅗ', 'ㅛ', 'ㅜ', 'ㅠ', 'ㅡ', 'ㅣ',
];

fn main() -> io::Result<()> {
    let file = File::open("./input")?;
    let reader = BufReader::new(file);
    let mut hash: HashMap<String, u8> = HashMap::new();
    let mut now: u8 = 0;
    let mut state: State = State::Ready;
    for line in reader.lines() {
        let seq = line?.trim().to_string();
        match state {
            State::GetInput(sz) => {
                if sz == 0 {
                    now += 1;
                    state = State::GetInput(seq.parse().unwrap());
                } else {
                    hash.insert(seq, now);
                    state = State::GetInput(sz - 1);
                }
            }
            _ => {
                state = State::GetInput(seq.parse().unwrap());
            }
        }
    }
    let (stdout, stdin) = (io::stdout(), io::stdin());
    let (mut sout, mut sin) = (
        BufWriter::new(stdout.lock()),
        UnsafeScanner::new(stdin.lock()),
    );
    let mut korean_vec: Vec<u8> = Vec::new();
    loop {
        let seq: String = sin.token();
        if seq == "END" {
            break;
        }
        let mut trans: Vec<char> = seq.chars().collect();
        for i in 0..3 {
            let c = trans[i].clone();
            trans[i] = if c == 'A' {
                'U'
            } else if c == 'G' { 'C' } else if c == 'C' { 'G' } else { 'A' };
        }
        let s: String = trans.iter().collect();
        if let Some(&res) = hash.get(&s) {
            korean_vec.push(res);
        }
    }
    for v in korean_vec {
        write!(sout, "{} ", KOREAN_TABLE[v as usize]).ok();
    }
    Ok(())
}
