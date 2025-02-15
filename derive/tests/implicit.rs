// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
extern crate pest;
extern crate pest_derive;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../tests/implicit.pest"]
struct TestImplicitParser;

#[test]
fn test_implicit_whitespace() {
    // this failed to parse due to a bug in the optimizer
    // see: https://github.com/pest-parser/pest/issues/762#issuecomment-1375374868
    let successful_parse = TestImplicitParser::parse(Rule::program, "a a");
    assert!(successful_parse.is_ok());
}
