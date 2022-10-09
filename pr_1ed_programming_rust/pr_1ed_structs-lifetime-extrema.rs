
/// Structs with lifetime parameters
/// hold references to the greatest and least elements of some slice
struct Extrema<'elt> {
  greatest: &'elt i32,
  least: &'elt i32,
}

fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
//fn find_extrema<'s>(slice: &[i32]) -> Extrema {
  let mut greatest  = &slice[0];
  let mut least     = &slice[0];

  for i in 1..slice.len() {
    if slice[i] < *least    { least     = &slice[i]; }
    if slice[i] > *greatest { greatest  = &slice[i]; }
  }
  Extrema { greatest, least }
}

fn main() {

  let array = [0, -3, 0, 15, 48];
  let extr  = find_extrema(&array);
  assert_eq!(*extr.least, -3);
  assert_eq!(*extr.greatest, 48);

}