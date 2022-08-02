use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Solution {}

struct ElementRow {
    pub element: i32,
    pub row: usize,
}

impl PartialEq for ElementRow {
    fn eq(&self, other: &Self) -> bool {
        self.element == other.element && self.row == other.row
    }
}

impl PartialOrd for ElementRow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.element.cmp(&other.element) {
            // reverse ordering for min_heap
            Ordering::Less => Some(Ordering::Greater),
            Ordering::Greater => Some(Ordering::Less),
            Ordering::Equal => None,
        }
    }
}

impl Eq for ElementRow {}
impl Ord for ElementRow {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(ordering) = self.partial_cmp(other) {
            return ordering;
        }
        Ordering::Equal
    }
}

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let first_col = matrix
            .iter()
            .enumerate()
            .map(|(index, row)| ElementRow {
                element: row[0],
                row: index,
            })
            .collect::<Vec<ElementRow>>();

        let mut col_tracker: Vec<usize> = vec![1; matrix.len()];

        let mut min_heap: BinaryHeap<ElementRow> = BinaryHeap::from(first_col);

        for _ in 1..k {
            if let Some(er) = min_heap.pop() {
                if col_tracker[er.row] != matrix[0].len() {
                    // we have visited all elements of this row yet
                    min_heap.push(ElementRow {
                        element: matrix[er.row][col_tracker[er.row]],
                        row: er.row,
                    });
                    col_tracker[er.row] += 1;
                }
            }
        }

        min_heap.peek().unwrap().element
    }
}

use std::env;

fn main() {
    let matrix = vec![vec![1, 5, 9], vec![10, 11, 13], vec![13, 13, 15]];

    let args: Vec<String> = env::args().collect();

    let k = match args.get(1) {
        Some(k_string) => match k_string.parse::<i32>() {
            Ok(ki32) => ki32,
            _ => 8,
        },
        None => 8,
    };

    println!("{:?}", Solution::kth_smallest(matrix, k));
}
