struct Array<T, const N: usize> {
    data: [T; N],
}

fn main() {
    let arrays = [
        Array { data: [1, 2, 3], },
        Array { data: [1.0, 2, 3], },
        Array { data: [1, 2], },
    ];
}
