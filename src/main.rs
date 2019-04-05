
fn main() {
    // let i = 13u8;
    // let b = 0b11_1000;
    // let h = 0xf5;
    // let f1 = 1 / 3;
    // let f2 = 1.0 / 3;
    // let f3 = 1 / 3.0;
    let f4: f32 = 1.0 / 3.0;
    let f5: f64 = 1.0 / 3.0;

    let f6: f64 = f4.into();
    // let f7: f32 = f5;


    println!("i: {}, {}, {}", f4, f5, f6);
}