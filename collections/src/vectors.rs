use std::collections::HashMap;

// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

pub fn median(vector: &[i32]) -> Option<i32> {
    let len: usize = vector.len();
    if len < 1 {
        return None;
    }

    let sorted = selection_sort(vector);

    println!("{:?}", sorted);

    let half = len / 2;

    if len % 2 != 0 {
        return Some(sorted[half]);
    } else {
        return Some((sorted[half] + sorted[half - 1]) / 2);
    }
}

pub fn mode(vector: &[i32]) -> Option<i32> {
    if vector.is_empty() {
        return None;
    }

    let mut map = HashMap::new();
    let mut max_key: i32 = vector[0];
    let mut max: i32 = 1;
    
    for i in vector.iter() {
        let p = map.entry(*i).or_insert(0);
        *p += 1;
        if *p > max {
            max = *p;
            max_key = *i;
        }
    }

    return Some(max_key);
}

fn selection_sort(vector: &[i32]) -> Vec<i32> {
    let len = vector.len();
    let mut sorted: Vec<i32> = vector.to_vec();

    for i in 0..(len - 1) {
        let mut smallest = sorted[i];
        let mut smallest_index = i;

        for j in (i + 1)..len {
            if sorted[j] < smallest {
                smallest = sorted[j];
                smallest_index = j;
            }
        }

        sorted.swap(smallest_index, i);
    }

    return sorted;
}
