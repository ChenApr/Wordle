use std::collections::HashSet;

use crate::builtin_words;
use crate::builtin_words::*;

use rand::seq::SliceRandom;
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};

pub enum Error {
    FileNotExist,
    DuplicatedWords,
    BeyondBuiltinWords
}

//This builds a bank to store the real FINAL and ACCEPTABLE words in a game
#[derive(Deserialize, Serialize)]
pub struct Words {
    pub FINAL: Vec<String>,
    pub ACCEPTABLE: Vec<String>
}

//Inspired by ChatGPT
//Exam if there are duplicated words in a word-set
fn to_hashset(_words: &Vec<String>) -> Result<HashSet<String>, Error>{
    let mut hashset: HashSet<String> = HashSet::new();
    for i in _words {
        if !hashset.insert(i.to_string()) {
            return Err(Error::DuplicatedWords);
        }
    }
    Ok(hashset)
}

impl Words {

    //Default: Setting the word_bank to the builtin_word.rs
    pub fn new() -> Words{
        let _final = FINAL.iter().map(|s| s.to_string()).collect();
        let _acceptable = ACCEPTABLE.iter().map(|s| s.to_string()).collect();
        Words{FINAL: _final, ACCEPTABLE: _acceptable}
    }

    pub fn FINAL_set(&mut self, _final_set: String) -> Result<(),Error>{
        //read from file provided
        if let Ok(mut f) = std::fs::File::open(_final_set) {

            let mut _content = String::new();
            std::io::Read::read_to_string(&mut f, &mut _content).unwrap();

            let mut _content: Vec<String> = _content.lines().map(|s| s.to_string().to_lowercase()).collect();

            //Exam if there are words beyond the builtin_words
            match to_hashset(&_content) {
                Ok(set) => {

                    let ori_set: HashSet<String> = FINAL.iter().map(|s| s.to_string()).collect();
                    
                    if !set.is_subset(&ori_set) {
                        return Err(Error::BeyondBuiltinWords)
                    }
                    else {

                        //renew the FINAL and sorted
                        _content.sort_by(|a, b| a.cmp(&b));
                        self.FINAL = _content;
                        return Ok(())
                    }
                
                }

                Err(Error::DuplicatedWords) => return Err(Error::DuplicatedWords),

                _ => unimplemented!()
            }

            Ok(())
        }
        else {
            Err(Error::FileNotExist)
        }
    }

    //Same logic with FINAL_set
    pub fn ACCEPTABLE_set(&mut self, _acceptable_set: String) -> Result<(),Error>{
        if let Ok(mut f) = std::fs::File::open(_acceptable_set) {

            let mut _content = String::new();
            std::io::Read::read_to_string(&mut f, &mut _content).unwrap();

            let mut _content: Vec<String> = _content.lines().map(|s| s.to_string().to_lowercase()).collect();

            match to_hashset(&_content) {
                Ok(set) => {

                    let ori_set: HashSet<String> = ACCEPTABLE.iter().map(|s| s.to_string()).collect();
                    
                    if !set.is_subset(&ori_set) {
                        return Err(Error::BeyondBuiltinWords)
                    }
                    else {
                        _content.sort_by(|a, b| a.cmp(&b));
                        self.ACCEPTABLE = _content;
                        return Ok(())
                    }
                
                }

                Err(Error::DuplicatedWords) => return Err(Error::DuplicatedWords),

                _ => unimplemented!()
            }

            Ok(())
        }
        else {
            Err(Error::FileNotExist)
        }
    }


    //A rand generator, updating the FINAL at the same time
    pub fn FINAL_RAND(&mut self, seed: u64) {
    
        //Initializing a rand generator using the seed
        let mut rng = StdRng::seed_from_u64(seed);
    
        //Copy the FINAL and shuffle it
        let mut candidates: Vec<String> = self.FINAL.iter().map(|s| s.to_string()).collect();
    
        candidates.shuffle(&mut rng);
    
        self.FINAL = candidates;
    }

}

