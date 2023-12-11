//# rayon = "*"
use rayon::prelude::*;
fn main() {
    let ids = vec![
        " 1234", "5678 ", " 1155 ", "555.5",
        "-9999", "5", "twelve", "1001", "9999"
    ];
    let ids = ids
        .iter()
        .map(|id| id.trim())
        .filter_map(|id| id.parse().ok())
        .filter(|num| num >= &1000)
        .collect::<Vec<usize>>();
    println!("{ids:?}");

    let ids = vec![
        " 1234", "5678 ", " 1155 ", "555.5",
        "-9999", "5", "twelve", "1001", "9999"
    ];
    let ids = ids
        .par_iter() // rayon
        .map(|id| id.trim())
        .filter_map(|id| id.parse().ok())
        .filter(|num| num >= &1000)
        .collect::<Vec<usize>>();
    println!("{ids:?}");

    let mut ids: Vec<usize> = ids;
    ids.par_sort();
    for id in ids {
        println!("{id}");
    }

    let ids = vec![
        " 1234", "5678 ", " 1155 ", "555.5",
        "-9999", "5", "twelve", "1001", "9999"
    ];
    ids.par_iter()
        .map(|id| id.trim())
        .for_each(|id| println!("{id}"));
}
