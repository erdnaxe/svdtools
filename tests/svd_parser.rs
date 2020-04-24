use std::path::Path;
use svdtools::common::svd_reader;

// test svd parser lib consistency
#[test]
fn read_and_write() {
    // read an svd
    let res_dir = Path::new("res/example1");
    let svd_path = res_dir.join("stm32l4x2.svd");
    let svd = svd_reader::device(&svd_path).unwrap();

    // write the svd in xml
    let xml = svd_parser::encode(&svd).unwrap();

    // read again the svd
    let _same_svd = svd_parser::parse(&xml).unwrap();

    // TODO assert_eq when https://github.com/rust-embedded/svd/issues/111
    //      is solved
}
