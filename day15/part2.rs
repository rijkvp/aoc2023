mod part1;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut boxes = vec![Vec::<(&str, u64)>::new(); 256];
    for part in input.trim().split(',') {
        if part.contains('=') {
            let (label, focal_length) = part.split_once('=').unwrap();
            let focal_length = focal_length.parse::<u64>().unwrap();
            let box_idx = part1::hash(label) as usize;
            if let Some(pos) = boxes[box_idx].iter().position(|&(l, _)| l == label) {
                boxes[box_idx][pos] = (label, focal_length); // Replace existing
            } else {
                boxes[box_idx].push((label, focal_length)); // Add to front
            }
        } else {
            let label = &part[..part.len() - 1];
            let box_idx = part1::hash(label) as usize;
            boxes[box_idx].retain(|&(l, _)| l != label);
        }
    }
    let mut sum = 0;
    for (box_n, box_list) in boxes.iter().enumerate() {
        for (slot_n, (_, focal_length)) in box_list.iter().enumerate() {
            sum += (box_n + 1) as u64 * (slot_n + 1) as u64 * focal_length;
        }
    }
    println!("{sum}");
}
