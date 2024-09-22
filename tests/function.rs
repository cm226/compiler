use std::io::Error;

mod util;

#[test]
fn function_test() -> Result<(), Error>{
    let expected = "1\0\n\02\00\0\n\0";
    util::run_test("functions", expected)?;
    Ok(())
}