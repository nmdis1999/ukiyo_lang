extern crate ukiyo;
fn main() {
    // let lexerdef = ukiyo_l::lexerdef();
    // let stdin = io::stdin();
    // loop {
    //     print!(">>> ");
    //     io::stdout().flush().ok();
    //     match stdin.lock().lines().next() {
    //         Some(Ok(ref l)) => {
    //             if l.trim().is_empty() {
    //                 continue;
    //             }
    //             // Now we create a lexer with the `lexer` method with which
    //             // we can lex an input.
    //             let lexer = lexerdef.lexer(l);
    //             // Pass the lexer to the parser and lex and parse the input.
    //             let (res, errs) = ukiyo_y::parse(&lexer);
    //             for e in errs {
    //                 println!("{}", e.pp(&lexer, &ukiyo_y::token_epp));
    //             }
    //             match res {
    //                 Some(Ok(r)) => {
    //                     //println!("Result: {:?}", r)
    //                     run(r, &lexer);
    //                 }
    //                 _ => eprintln!("Unable to evaluate expression."),
    //             }
    //         }
    //         _ => {
    //             break;
    //         }
    //     }
    // }
    ukiyo::mainv2::main()
}