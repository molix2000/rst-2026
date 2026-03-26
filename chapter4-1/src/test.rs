extern crate test;

#[bench]
fn bench_string_concat(b: &mut test::Bencher) {
     b.iter( || {
         let mut s = String::from("Hello");
         s.push_str(", world");
    });
}
