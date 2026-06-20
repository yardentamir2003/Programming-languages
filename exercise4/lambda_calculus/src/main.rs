// Declare modules
mod utils;
mod lexer;
mod parser;
mod reducer;

use parser::{Term, parse};
use reducer::{reduce_cbv, reduce_cbn};

/// Recursive evaluation helper.
/// Runs the reducer until None is returned.
fn evaluate(reduce_func: fn(&Term) -> Option<Term>, t: Term) {
    print!("{}", t);
    match reduce_func(&t) {
        None => {
            println!(" =/=>\n");
        },
        Some(t_prime) => {
            print!(" ==>\n\n");
            evaluate(reduce_func, t_prime);
        }
    }
}

fn main() {
    let s1 = "
    let tru = (\\t.(\\f.t)) in
    let fls = (\\t.(\\f.f)) in
    let and = (\\b.(\\c. ((b c) fls))) in
    ((and tru) tru)
    ";

    let s2 = "
    let tru = (\\t.(\\f.t)) in
    let fls = (\\t.(\\f.f)) in
    let and = (\\b.(\\c. ((b c) fls))) in
    ((and fls) tru)
    ";

    let s3 = "((\\id1.(t1 id1)) (\\id2.(t1 t2)))";

    let s4 = "(((\\id1.(t1 id1)) (\\id2.(t1 t2))) ((\\id1.(t1 id1)) (\\id2.(t1 t2))))";

    let s5 = "((\\id1.(t1 id1)) (\\id1.(t1)))";

    let s6 = "
    let tru = (\\t.(\\f.t)) in
    let fls = (\\t.(\\f.f)) in
    let and = (\\b.(\\c. ((b c) fls))) in
    let not = (\\x.((x fls) tru)) in
    (((not and) fls) tru)
    ";


    // New tests

    let s7 = "
    let tru = (\\t.(\\f.t)) in
    let fls = (\\t.(\\f.f)) in
    let or = (\\b.(\\c.((b tru) c))) in
    ((or fls) tru)
    ";


    let s8 = "((\\x.(\\y.x)) (\\w.y))";


    let s9 = "((\\x.(\\y.y)) ((\\x.(x x)) (\\x.(x x))))";

    println!("\nEvaluating:\n{}\nin cbn semantics:\n", s1);
    evaluate(reduce_cbn, parse(s1));

    println!("\n\nEvaluating:\n{}\nin cbv semantics:\n", s2);
    evaluate(reduce_cbv, parse(s2));

    println!("\n\n Testing on:\n{}\nReduce cbv\n", s3);
    evaluate(reduce_cbv, parse(s3));

    println!("\n\n Testing on:\n{}\nReduce cbn\n", s3);
    evaluate(reduce_cbn, parse(s3));
    
    println!("\n\n Testing on:\n{}\nReduce cbv\n", s5);
    evaluate(reduce_cbv, parse(s5));

    println!("\n\n Testing on:\n{}\nReduce cbv\n", s6);
    evaluate(reduce_cbv, parse(s6));

    println!("\n\n Testing on:\n{}\nReduce cbn\n", s6);
    evaluate(reduce_cbn, parse(s6));

    // Run new tests

    println!("\n\n=== Custom Test 1: Logical OR ===");
    println!("Evaluating:\n{}\nin cbv semantics:\n", s7);
    evaluate(reduce_cbv, parse(s7));

    println!("\n\n=== Custom Test 2: Alpha Conversion Avoids Capture ===");
    println!("Evaluating:\n{}\nin cbv semantics:\n", s8);
    evaluate(reduce_cbv, parse(s8));

    println!("\n\n=== Custom Test 3: CBN terminates on Omega ===");
    println!("Evaluating:\n{}\nin cbn semantics:\n", s9);
    evaluate(reduce_cbn, parse(s9));
    
}