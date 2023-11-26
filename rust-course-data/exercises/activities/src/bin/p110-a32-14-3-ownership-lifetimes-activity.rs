// Topic: Lifetimes & Structures
//
// Requirements:
// * Display just the names and titles of persons from the mock-data.csv file
// * The names & titles must be stored in a struct separately from the mock
//   data for potential later usage
// * None of the mock data may be duplicated in memory
//
// Notes:
// * The mock data has already been loaded with the include_str! macro, so all functionality
//   must be implemented using references/borrows

const MOCK_DATA: &'static str = include_str!("mock-data.csv");

struct Names<'a> { inner: Vec<&'a str>, }
struct Titles<'a> { inner: Vec<&'a str>, }
fn main() {
    let data: Vec<_> = MOCK_DATA.split('\n').skip(1).collect();
    let names: Vec<_> = data.iter().filter_map(|line| line.split(',').nth(1)).collect();
    let names = Names { inner: names };
    let titles: Vec<_> = data.iter().filter_map(|line| line.split(',').nth(4)).collect();
    let titles = Titles { inner: titles };
    for n in names.inner.iter().take(3) { println!("{n:?}"); }
    for n in titles.inner.iter().take(3) { println!("{n:?}"); }

    let data = names.inner.iter().zip(titles.inner.iter());
    for (name, title) in data.take(5) { println!("Name: {name:?}, Title: {title:?}"); }
}
