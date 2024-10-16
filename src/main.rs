// -------------------------------------------------------------------------------------------------

// Theorems

// Every G = K(m, n) graph with cr(G) = k contains a drawing of H = K(m-1, n) with cr(H) <= k(m-2)/m

// If m is even and the Zarankiewicz’ conjecture holds for K(m−1, n), then it holds for K(m, n).

// If m and n are odd and m0 < m is even, such that the Zarankiewicz’ conjecture holds for K(m0 + 1, n) and K(m − m0, n),
// then in any drawing of K(m, n) that includes a drawing of K(m0, n) with Z(m0)Z(n) or fewer crossings,
// there are at least Z(m)Z(n) crossings.

// (a) If a ∈ V (COn), then d(a, a¯) = Z(n).
// (b) Every (3, n) set has antisum at least Z(n).
// (c) If m and n are both odd, then the antisum of a (m, n) set is odd when m ≡ n ≡ 3 (mod 4) and even otherwise.

// Let m and n be odd integers and m0 < m be even so that
// every (m0 + 1, n)-set has antisum at least Z(m0 + 1)Z(n) and
// every (m − m0, n)-set has antisum at least Z(m − m0)Z(n).
// Then, if an (m, n)-set contains an (m0, n) subset with antisum Z(m0)Z(n) or less it has antisum at least Z(m)Z(n).

// -------------------------------------------------------------------------------------------------

// Goal: Check if there exists a (m, n) set with antisum smaller than Z(m)Z(n).

// Caveat: We want to limit what sets we look at by eliminating isomorphic duplicates ((m-1)!)^n possible sets

mod hazelcast;
use hazelcast::*;
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::Read,
    path::Path,
};

type Letter = [u8; N];
type Word = Vec<Letter>;

const M: usize = 9;
const N: usize = 9;

const CLIENT: Lazy<HazelcastRestClient> =
    Lazy::new(|| HazelcastRestClient::new("127.0.0.1", "5701"));

// Lexicographically smallest letter
const BASE: Letter = [0, 1, 2, 3, 4, 5, 6, 7, 8];

// Max lobspacron values for (9,9), for k = 2 up to k = 9
const F_MAXS: [u32; 8] = [6, 20, 40, 68, 104, 146, 197, 254];

// Calculates the Zarankiewicz number for a graph
fn zarankiewicz_number(num_nodes: u32) -> u32 {
    let f = num_nodes / 2u32;
    let g = (num_nodes - 1) / 2u32;
    return f * g;
}

// Calculate the crossing number as conjectured by Zarankiewicz
fn zarankiewicz_conjecture(left_nodes: u32, right_nodes: u32) -> u32 {
    let left = zarankiewicz_number(left_nodes);
    let right = zarankiewicz_number(right_nodes);
    return left * right;
}

fn antisum(multiset: &Word) -> u32 {
    if multiset.len() == 1 {
        return 0;
    }

    multiset
        .iter()
        .combinations(2)
        .map(|vec| antidistance(vec[0], vec[1]))
        .sum()
}

fn get_neighbors(node: Letter) -> Vec<Letter> {
    let mut neighbors = Vec::with_capacity(N - 1);

    let max_idx: usize = node.len() - 1;

    // Do first swap with first and second element swapping
    // Has to rotate left so that leftmost element is always the lexicographical smallest
    neighbors.push({
        let mut neighbor = node.clone();
        neighbor.swap(0, 1);
        neighbor.rotate_left(1);
        neighbor
    });

    // Do all possible easy swaps
    for idx in 1..max_idx {
        neighbors.push({
            let mut neighbor = node.clone();
            neighbor.swap(idx, idx + 1);
            neighbor
        });
    }

    // Do final swap with first and last element swapping
    // Has to rotate right so that leftmost element is always the lexicographical smallest
    neighbors.push({
        let mut neighbor = node.clone();
        neighbor.rotate_right(1);
        neighbor.swap(0, 1);
        neighbor
    });

    neighbors
}

trait ToLetter {
    fn to_letter(&self) -> Letter;
}

trait ToWord {
    fn to_word(&self) -> Word;
}

impl ToWord for String {
    fn to_word(&self) -> Word {
        self.split(" ")
            .map(|letter| letter.to_string().to_letter())
            .collect()
    }
}

impl ToLetter for String {
    fn to_letter(&self) -> Letter {
        self.chars()
            .map(|c| c as u8 - '0' as u8)
            .collect_vec()
            .try_into()
            .expect("Converting string to Letter")
    }
}

trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for Letter {
    fn to_string(&self) -> String {
        self.iter().map(|&num| (num + '0' as u8) as char).collect()
    }
}

impl ToString for Word {
    fn to_string(&self) -> String {
        self.iter().map(|letter| letter.to_string()).join(" ")
    }
}

fn build_map() -> HashMap<Letter, u32> {
    // Read prebuilt map if it exists
    let path = Path::new("distance_map.json");
    let maybe_file = File::open(&path);
    if let Ok(mut file) = maybe_file {
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Reading map from file");
        let map = serde_json::from_str::<HashMap<String, u32>>(&buf).expect("Invalid hashmap");
        let mut letter_map = HashMap::new();
        for (string, distance) in map {
            letter_map.entry(string.to_letter()).or_insert(distance);
        }
        return letter_map;
    }

    // Generate the map

    let mut distance_map: HashMap<Letter, u32> = HashMap::new();

    let mut queue: VecDeque<Letter> = VecDeque::new();
    queue.push_back(BASE);

    // The current distance a given node is at
    let mut distance: u32 = 0;

    // Used to see when distance needs to be increased by one
    let mut last_in_layer: usize = 1;

    while queue.len() > 0 {
        let node = queue.pop_front().expect("Cannot pop from empty queue");

        // Get neighboring nodes not already viewed
        let all_neighbors = get_neighbors(node);
        let neighbors = all_neighbors
            .iter()
            .filter(|x| !(distance_map.contains_key(*x) || queue.contains(x)))
            .collect_vec();

        neighbors.iter().for_each(|x| queue.push_back(**x));

        // Add distance to map
        distance_map.entry(node).or_insert(distance);

        last_in_layer -= 1;
        // increase distance
        if last_in_layer == 0 {
            distance += 1;
            println!("Now on distance {distance}");
            println!("Queue is {} long", queue.len());
            last_in_layer = queue.len();
        }
    }

    // Write to file for future reads
    if maybe_file.is_err() {
        let mut map: HashMap<String, u32> = HashMap::new();
        for (letter, distance) in distance_map.clone() {
            map.entry(letter.to_string()).or_insert(distance);
        }
        let file = File::create(path).expect("Creating map file");
        serde_json::to_writer(file, &map).expect("Writing map to file");
    }

    distance_map
}

fn antidistance(left: &Letter, right: &Letter) -> u32 {
    // The map has letters as keys and distances as values.
    // The map assumes you are comparing "012345678" to the passed in key
    // This is calculated by using letter remapping.
    static DISTANCE_MAP: Lazy<HashMap<Letter, u32>> = Lazy::<HashMap<Letter, u32>>::new(build_map);

    if left == right {
        return zarankiewicz_number(left.len() as u32);
    }

    // Override left so we can just calculate regular distance
    let left = &antipode(left);

    if left == right {
        return 0;
    }

    // Rename elements so that left is sorted
    let mapping = map_to_base(left);

    let mapped_right: Letter = {
        let mut arr = BASE.clone();
        for (idx, ele) in right.iter().enumerate() {
            arr[idx] = *mapping.get(ele).unwrap();
        }

        let idx = arr
            .iter()
            .position(|&num| num == 0)
            .expect("Could not find element '0' in array");
        arr.rotate_left(idx);
        arr
    };

    *DISTANCE_MAP.get(&mapped_right).unwrap_or_else(|| {
        panic!(
            "Expected letter {:?} to be in the map, but it was not!",
            mapped_right
        )
    })
}

// Returns a u8 -> u8 mapping such that the provided letter is mapped to "012345678"
fn map_to_base(letter: &Letter) -> HashMap<&u8, u8> {
    let mut mapping = HashMap::new();
    for (idx, ele) in letter.iter().enumerate() {
        mapping.entry(ele).or_insert(idx as u8);
    }

    return mapping;
}

fn map_to_letter<'a>(from: &'a Letter, to: &Letter) -> HashMap<&'a u8, u8> {
    let mut mapping = HashMap::new();
    for (key, value) in from.iter().zip(to) {
        mapping.entry(key).or_insert(*value);
    }

    return mapping;
}

fn antipode(node: &Letter) -> Letter {
    let mut vec: Letter = BASE.clone();
    let mut iter = node.iter();
    // Don't reverse the 'a' at the start
    iter.next();
    for (idx, ele) in iter.rev().enumerate() {
        // Prevent index overflow
        if idx == N {
            break;
        }
        vec[idx + 1] = ele.clone();
    }

    return vec;
}

// whether f( new word ) ≤ fMAX( l + 1 )
// Returns the new antisum, provided the new letter is valid
fn check1(prev_word: &Word, new_letter: &Letter, prev_antisum: u32) -> bool {
    let k = prev_word.len();

    // No (7,9) or (9,9)-set can have a (4,9) subset with antisum <= 32
    if k == 4 && prev_antisum <= 32 {
        return false;
    }

    let a = prev_word[0];

    // Ordering is NOT preserved.
    // DO NOT USE AS A WORD
    let all_but_a = {
        let mut word = prev_word.clone();
        word.swap_remove(0);
        word
    };

    let af: u32 = antidistance(&prev_word[0], new_letter);
    let ax: i32 = all_but_a.iter().map(|x| antidistance(&a, x)).sum::<u32>() as i32;
    let xf: i32 = prev_word
        .iter()
        .map(|x| antidistance(x, new_letter))
        .sum::<u32>() as i32;

    if (af as i32) < prev_antisum as i32 + ax - ((k as i32 - 2) * xf) {
        return false;
    } else if (af as i32) > F_MAXS[k + 1] as i32 - prev_antisum as i32 - ax {
        return false;
    }

    let mut new_word = prev_word.clone();
    new_word.push(*new_letter);

    let new_antisum = antisum(&new_word);

    // No (9,9)-set can have a (4,9) subset with antisum <= 32
    if k + 1 == 4 && new_antisum <= 32 {
        return false;
    }

    return true;
}

// Checks for better words that are isomorphic to it
fn check2(word: &Word) -> Vec<Word> {
    let mut better_words_antisum: Vec<Word> = vec![];
    let mut better_words_lexicographically: Vec<Word> = vec![];

    // Check reorderings of the word for smaller lobspacron value
    for (idx, _) in word.iter().enumerate() {
        let (elements, last_letter) = {
            let mut modifiable_word = word.clone();
            let removed = modifiable_word.swap_remove(idx);
            (modifiable_word, removed)
        };

        let permutation_size = elements.len();

        for mut new_word in elements.into_iter().permutations(permutation_size) {
            new_word.push(last_letter);
            let cmp = antisum(&new_word) as i32 - antisum(word) as i32;
            if cmp > 0 {
                // new antisum is greater, reject ordering
                continue;
            } else if cmp < 0 {
                // new antisum lesser, reject word
                better_words_antisum.push(new_word);
                continue;
            } else if better_words_antisum.is_empty() {
                // new antisum the same, run mapping
                println!("This is where I'd check mappings for better lexicographical elements");
                if &new_word == word {
                    continue;
                }
                better_words_lexicographically.push(new_word);
            }
        }
    }

    if better_words_antisum.is_empty() {
        better_words_lexicographically
    } else {
        better_words_antisum
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let host = args.iter().any(|arg| arg == "host");

    if host {
        // We generate entries iteratively, starting from 012345678, adding Letters as they fit, splitting off where we need to.
        let base_word = vec![BASE];

        let prev_word = &base_word;
        for letter in BASE.clone().into_iter().permutations(N) {
            let letter = vec_to_arr::<u8, 9>(letter);
            if check1(prev_word, &letter, antisum(prev_word)) {
                // TODO do check2

                let new_word = {
                    let mut temp = prev_word.clone();
                    temp.push(letter);
                    temp
                };
                let _ = CLIENT.queue_offer("to_check", new_word.to_string());
            }
        }
    } else {
        loop {
            let res = CLIENT.queue_delete("to_check", 3);
            match res {
                Ok(to_check) => {
                    let word = to_check.to_word();
                    let better_words = check2(&word);
                    println!("{better_words:?}");
                }
                Err(err) => panic!("Unexpected error! {}", err),
            }
        }
    }
}

fn vec_to_arr<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}
