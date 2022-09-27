// [如何在Rust中打印变量的类型及类型转换实践](http://wjhsh.net/pu369-p-15163424.html)

#![feature(core_intrinsics)]
fn print_type_of<T>(_: T) {
    println!("{}", std::intrinsics::type_name::<T>());
}

fn main() {
    print_type_of(3);       // prints "i32"
    print_type_of(32.90);   // prints "f64"
    print_type_of(vec![102, 111, 111]);                // prints "alloc::vec::Vec<i32>"
    print_type_of(b"foo12".to_vec());            // prints "alloc::vec::Vec<u8>" 
    print_type_of(b"foo12".to_owned());            // prints "[u8; 5]"  
    print_type_of( "bar".as_bytes().to_vec());   // prints "alloc::vec::Vec<u8>"   
    print_type_of( "bar".as_bytes().to_owned());   // prints "alloc::vec::Vec<u8>"   
    println!("===================================================");
    print_type_of("foo");            // prints "&str"   
    let s = "bar";       
    print_type_of(&s);               // prints "&&str"   
    print_type_of(*&s);               // prints "&str"  
    print_type_of(&&s);               // prints "&&&str"    
    print_type_of([0x08, 0x09, 0x0a, 0x0b, ]);           // prints "[i32; 4]"   
    print_type_of([0x08, 0x09, 0x0a, 0x0b, 0x0b,]);      // prints "[i32; 5]"   
    print_type_of("foo".as_bytes());                     // prints "&[u8]"   
    print_type_of(b"foo15");                             // prints "&[u8; 5]"    
    print_type_of("foo".to_string());    // prints "alloc::string::String"  
    print_type_of("foo".to_owned());     // prints "alloc::string::String"   
    print_type_of(String::from("foo"));  // prints "alloc::string::String"    
    print_type_of(String::from_utf8(vec![102, 111, 111]).unwrap());          //prints "alloc::string::String"  
    println!("{}",String::from_utf8(vec![102, 111, 111]).unwrap());          //prints "foo"  
    print_type_of(std::str::from_utf8(&[0x66, 0x6F, 0x6F,]).unwrap());       //prints "&str"  
    println!("{}",std::str::from_utf8(&[0x66, 0x6F, 0x6F,]).unwrap());       //prints "foo"  
    println!("{}",std::str::from_utf8(&[102, 111, 111,]).unwrap());          //prints "foo"  
    println!("{}",std::str::from_utf8( "foo".as_bytes()).unwrap());          //prints "foo" 
    println!("{}",std::str::from_utf8(&vec![102, 111, 111]).unwrap());       //prints "foo" 
    print_type_of(vec![102, 111, 111].as_slice());       //prints "&[i32]" 
    println!("{:?}",vec![102, 111, 111].as_slice());     //prints "[102, 111, 111]"

    println!("===================================================");

    //字符串&str
    let v0: &str ="我";
    //&str->char
    let v1:char = v0.chars().nth(0).unwrap(); 
    //char ->i32
    let v2:i32 =v1 as i32; 
    //&str->&[u8]
    let v3:&[u8] =v0.as_bytes();
    //&[u8]->&str
    let v4:&str = std::str::from_utf8(v3).unwrap();
    // i32->char
    let v5:char =char::from_u32(v2 as u32).unwrap();
    println!("v0_&str:{},v1_&str->char:{},v2_char ->i32:{}",v0,v1,v2);
    println!("v2_i32_HEX:{:X},v3_&str->&[u8]:{:?},v4_&[u8]->&str:{:?}",v2,v3,v4);
    println!("v5_i32->char:{}",v5);
}