// 8-bit	i8	u8
// 16-bit	i16	u16
// 32-bit	i32	u32
// 64-bit	i64	u64
// arch	isize	usize

fn main() {
    let f: bool = false;
    let tup: (i32, f64, u8) = (500, 6.4, 1); // int 32 float 64 unsigned8
    let (mut x, y, z) = tup;


    println!("{}", f);
}