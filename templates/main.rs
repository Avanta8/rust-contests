//$JSON

#[allow(unused_imports)]
use algo_lib::{
    dbg,
    io::{input::Input, output::Output},
};

$SOLVE

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
