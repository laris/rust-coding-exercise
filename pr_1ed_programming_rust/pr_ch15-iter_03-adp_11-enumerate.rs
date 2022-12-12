fn main() {
    /*
    let mut pixels = vec![0; columns * rows];
    let threads = 8;
    let band_rows = rows / threads + 1;
    let bands: Vec<&mut [u8]> = pixels.chunks_mut(band_rows * columns).collect();

    for (i, band) in bands.into_iter().enumerate() {
        let top = band_rows * i;
        // start a thread to render rows `top..top + band_rows`
    }
    */
    // doc example
    let a = ["a", "b", "c"];
    let mut iter = a.iter().enumerate();
    assert_eq!(iter.next(), Some((0, &"a")));
    assert_eq!(iter.next(), Some((1, &"b")));
    assert_eq!(iter.next(), Some((2, &"c")));
    assert_eq!(iter.next(), None);

}
