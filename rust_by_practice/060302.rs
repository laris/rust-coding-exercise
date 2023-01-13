fn main() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];
    // 修改数字 `8` 让代码工作
    // 小提示: 切片和数组不一样，它是引用。如果是数组的话，那下面的 `assert!` 将会通过： '中'和'国'是char类型，char类型是Unicode编码，大小固定为4字节，两个字符为8字节。
    //assert!(std::mem::size_of_val(&slice) == 8);
    assert!(std::mem::size_of_val(&arr[0]) == 4);
    assert!(std::mem::size_of_val(&slice) == 16);
}
