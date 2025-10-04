fn main(){
   let a: i8 = -100; // (-128 to 127)
   let b: u8 = 255; // (0 to 255)
   let c: i32 = -5000;
   let d: u32 = 5000;

   let e: u64 = 100000;   // 64-bits memory

   let f = 42; // type inferred to i32
   let g = 42.23; // type inferred to f64

   println!("{}", a);
   println!("{}", b);
   println!("{}", b);
   println!("{}", d);
    println!("{}", e);
   println!("{}", f);
   println!("{}", g);
}