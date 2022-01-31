extern crate time;
use fxhash::FxHashSet;
// use rayon::iter::IntoParallelIterator;
// use rayon::prelude::*;

// !!!cmk run parallel
//      see https://rustwasm.github.io/docs/wasm-bindgen/examples/raytrace.html?highlight=panic#building-the-demo
// !!!cmk give sign that it is running
// !!!return one equation at a time
// !!!release
// !!!small
// !!! possible progress bar: https://medium.com/geekculture/rusting-javascript-with-webassembly-632405ba5a42
// !!! with 500 it pops up a "this page is slow" warning
// !!! add README.md etc

use wasm_bindgen::prelude::*;

// See https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm
// See https://rustwasm.github.io/book/game-of-life/code-size.html#shrinking-wasm-size
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn search(end: usize) -> String {
    // table.len < ((end-1)^5*4)^(1/5) = (end-1)*4^(1/5) < (end-1)*1.32
    let table_len = (1.32 * (end - 1) as f64) as usize;
    let mut fifth: Vec<u128> = vec![];
    //let mut valueset: HashSet<u128> = HashSet::new();
    let mut valueset: FxHashSet<u128> = Default::default();
    for i in 0..table_len {
        let value = (i as u128).pow(5);
        valueset.insert(value);
        fifth.push(value);
    }

    let count: String = (4..end)
        //.into_par_iter()
        .map(|n4| {
            let mut subcount: Vec<String> = vec![];
            let mut sum = fifth[n4];

            for n3 in 3..n4 {
                let fn3 = fifth[n3];
                sum += fn3;

                for n2 in 2..n3 {
                    let fn2 = fifth[n2];
                    sum += fn2;

                    for n1 in 1..n2 {
                        let fn1 = fifth[n1];
                        sum += fn1;

                        if valueset.contains(&sum) {
                            let by5: u128 = ((sum as f64).powf(0.2f64) + 0.5f64) as u128;
                            let found = format!("<p>{n1}<sup>5</sup>+{n2}<sup>5</sup>+{n3}<sup>5</sup>+{n4}<sup>5</sup>={by5}<sup>5</sup></p>");

                            // println!("{:?} seconds for whatever you did.", start.elapsed());
                            subcount.push(found);
                            // return;
                        }
                        sum -= fn1
                    }
                    sum -= fn2;
                }
                sum -= fn3
            }
            subcount
        })
        .flatten()
        .collect::<Vec<String>>().join("");

    // println!("{} found in {:?} seconds.", count, start.elapsed());
    return count; //format!("{}", count);
                  //     "{count}<p><em>elapsed time: </em></p>", //{time:?}

    // );

    //return format!("{}", end + 2);
}
