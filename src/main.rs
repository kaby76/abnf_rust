// Generated from trgen 0.23.21
use std::env;
use std::fs::File;
use std::io::{self, Read, BufRead, BufReader, Write};
use std::process;
use std::time::Instant;
use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::error_listener::{ErrorListener, ConsoleErrorListener};
use antlr4rust::input_stream::InputStream;
use antlr4rust::recognizer::Recognizer;
use antlr4rust::token::Token;
use std::fs;
mod abnf;
use abnf::AbnfLexer;
use abnf::AbnfParser;

/*
struct CustomErrorListener<W: Write> {
    errors: usize,
    quiet: bool,
    tee: bool,
    output: W,
}

impl<W: Write> CustomErrorListener<W> {
    fn new(quiet: bool, tee: bool, output: W) -> Self {
        CustomErrorListener { errors: 0, quiet, tee, output }
    }
}

impl<'a, W: Write> ErrorListener<'a> for CustomErrorListener<W> {
    fn syntax_error<T: Recognizer<'a, TF = Token<'a>>>(
        &self, recognizer: &T, offending_symbol: Option<&Token>, line: usize,
        column: usize, msg: &str
    ) {
        // We can't mutate in trait use self; workaround with RefCell if needed
        let mut this = self as *const _ as *mut CustomErrorListener<W>;
        unsafe { (*this).errors += 1; }

        if self.tee {
            writeln!(self.output, "line {}:{} {}", line, column, msg).ok();
        }
        if !self.quiet {
            eprintln!("line {}:{} {}", line, column, msg);
        }
    }

    fn report_ambiguity<'a, T: Parser<'a>>(
        &self,
	dfa: &antlr4rust::dfa::DFA,
        start_index: isize,
	stop_index: isize,
	exact: bool,
        ambig_alts: &antlr4rust::bitset::BitSet,
	config: &antlr4rust::atn_config_set::ATNConfigSet,
	parser: &mut T,
    ) {
        ConsoleErrorListener::default().report_ambiguity(recognizer, dfa, start_index, stop_index, exact, &ambig_alts, &config);
    }

    fn report_attempting_full_context<T: Recognizer<'a, TF = Token<'a>>>(
        &self, recognizer: &T, dfa: &antlr4rust::dfa::DFA,
        start_index: isize, stop_index: isize, conflicting_alts: antlr4rust::bitset::BitSet,
        config: antlr4rust::atn_config_set::ATNConfigSet
    ) {
        ConsoleErrorListener::default().report_attempting_full_context(recognizer, dfa, start_index, stop_index, &conflicting_alts, &config);
    }

    fn report_context_sensitivity<T: Recognizer<'a, TF = Token<'a>>>(
        &self, recognizer: &T, dfa: &antlr4rust::dfa::DFA,
        start_index: isize, stop_index: isize, prediction: usize,
        config: antlr4rust::atn_config_set::ATNConfigSet
    ) {
        ConsoleErrorListener::default().report_context_sensitivity(recognizer, dfa, start_index, stop_index, prediction, &config);
    }
}
*/

fn parse_input(
    input_name: &str,
    is_filename: bool,
    flags: &Flags,
) -> usize {
    let my_string_result = fs::read_to_string(input_name);
    let input = my_string_result.unwrap(); // Panics if Err
    let istream = InputStream::new(input.as_bytes());
    let lexer = AbnfLexer::new(istream);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AbnfParser::new(token_stream);

    let mut tee_file = flags.tee.then(|| File::create(format!("{}.errors", input_name)).unwrap());
    let tee_writer: &mut dyn Write = tee_file.as_mut().map(|f| f as _).unwrap_or(&mut io::sink());

//    let mut lexer_listener = CustomErrorListener::new(flags.quiet, flags.tee, tee_writer);
//    let mut parser_listener = CustomErrorListener::new(flags.quiet, flags.tee, tee_writer);

//    lexer.remove_error_listeners();
//    lexer.add_error_listener(&mut lexer_listener);
//    parser.remove_error_listeners();
//    parser.add_error_listener(&mut parser_listener);

    let start = Instant::now();
    let tree = parser.rule_().expect("parsing failed setup");
    let elapsed = start.elapsed();

//    let mut error_cnt = lexer_listener.errors + parser_listener.errors;

/*
    if flags.show_tree {
        let tree_str = tree.to_string_tree(&parser);
        if flags.tee {
            let mut f = File::create(format!("{}.tree", input_name)).unwrap();
            writeln!(f, "{}", tree_str).ok();
        } else {
//            eprintln!("{}", tree_str);
        }
    }
*/
/*
    if !flags.quiet {
        eprint!("{}Go {} {} {} {:.3}\n",
            flags.prefix, idx, input_name,
            if error_cnt > 0 { "fail" } else { "success" },
            elapsed.as_secs_f64()
        );
    }
*/

//    if error_cnt > 0 { 1 } else { 0 }

    0
}

struct Flags {
    inputs: Vec<String>,
    is_fns: Vec<bool>,
    prefix: String,
    show_tokens: bool,
    show_tree: bool,
    show_trace: bool, // not used (ANTRL Rust lacks SetTrace API)
    tee: bool,
    quiet: bool,
}

fn main() {
    let mut flags = Flags {
        inputs: Vec::new(),
        is_fns: Vec::new(),
        prefix: String::new(),
        show_tokens: false,
        show_tree: false,
        show_trace: false,
        tee: false,
        quiet: false,
    };

    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-tokens" => flags.show_tokens = true,
            "-tree" => flags.show_tree = true,
            "-prefix" => {
                i += 1;
                flags.prefix = format!("{} ", args[i]);
            }
            "-input" => {
                i += 1;
                flags.inputs.push(args[i].clone());
                flags.is_fns.push(false);
            }
            "-tee" => flags.tee = true,
            "-q" => flags.quiet = true,
            "-trace" => flags.show_trace = true,
            "-x" => {
                let stdin = io::stdin();
                for line in stdin.lock().lines() {
                    let ln = line.unwrap();
                    flags.inputs.push(ln);
                    flags.is_fns.push(true);
                }
            }
            other => {
                flags.inputs.push(other.to_string());
                flags.is_fns.push(true);
            }
        }
        i += 1;
    }

    if flags.inputs.is_empty() {
            process::exit(1);
    } else {
        let mut error_code = 0;
        for (idx, input) in flags.inputs.iter().enumerate() {
            let is_fn = flags.is_fns[idx];
	    let start_all = Instant::now();
	    parse_input(input, true, &flags);
	    let elapsed = start_all.elapsed();
	    if !flags.quiet {
		eprintln!("{}Total Time: {:.3}", flags.prefix, elapsed.as_secs_f64());
	    }
        }
        process::exit(error_code  as i32);
    }
}
