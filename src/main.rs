#[macro_use]
extern crate nom;
extern crate rustyline;

//-----------------//
// Basic Parsers
//-----------------//
named!(symbol<&str, char>, one_of!("!$%&|*+-/:<=?>@^_~#"));
named!(letter<&str, char>, one_of!("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"));
named!(digit<&str, char>, one_of!("0123456789"));
named!(spaces<&str, &str>, take_while!(|c| c == ' ' || c == '\n'));

//-----------------//
// Lisp Definitions
//-----------------//
#[derive(Debug, PartialEq)]
enum LispVal {
    LAtom(String),
    LList(Vec<LispVal>),
    LDottedList((Vec<LispVal>, Box<LispVal>)),
    LNumber(u32),
    LString(String),
    LBool(bool),
}
use self::LispVal::*;

//-----------------//
// Lisp Parsers
//-----------------//
named!(lisp_atom<&str, LispVal>,
    do_parse!(
        h: alt!(symbol | letter) >>
        b: many0!(complete!(alt!(symbol | letter | digit))) >>
        ({
            let mut h = h.to_string();
            let rest: String = b.iter().collect();
            h.push_str(rest.as_str());
            match h.as_str() {
                "#t" => LBool(true),
                "#f" => LBool(false),
                _ => LAtom(h),
            }
        })
    )
);
named!(lisp_number<&str, LispVal>,
    do_parse!(
        ds: many1!(complete!(digit)) >>
        ({
            let s: String = ds.iter().collect();
            LNumber(s.parse().unwrap())
        })
    )
);
named!(lisp_string<&str, LispVal>,
    do_parse!(
        char!('"') >>
        chars: many0!(none_of!("\"")) >>
        char!('"') >>
        (LString(chars.iter().collect()))
    )
);
named!(lisp_expr<&str, LispVal>, alt!(lisp_atom | lisp_number | lisp_string));

//-----------------//
// Main
//-----------------//
fn parse(s: &str) -> Result<LispVal, ParserError> {
    let (rem, val) = lisp_expr(s)?;
    //Ok(LAtom("".to_string()))
    Ok(val)
}

fn main() {
    println!("Welcome to Scheme REPL!");

    let mut rl = rustyline::Editor::<()>::new();
    while let Ok(line) = rl.readline("> ") {
        if line.is_empty() { continue; }
        rl.add_history_entry(&line);
        match parse(&line) {
            Ok(v) => println!("{:?}", v),
            Err(e) => handle_error(e),
        }
    }
}

//-----------------//
// Error handling
//-----------------//
type ParserError<'a> = nom::Err<(&'a str, nom::error::ErrorKind)>;

fn handle_error(err: ParserError) {
    match err {
        nom::Err::Error((s, e)) => println!("Error parsing \"{}\": {}", s, e.description()),
        nom::Err::Failure((s, e)) => println!("Failure parsing \"{}\": {}", s, e.description()),
        nom::Err::Incomplete(nom::Needed::Unknown) => println!("Incomplete parsing: unknown"),
        nom::Err::Incomplete(nom::Needed::Size(s)) => println!("Incomplete parsing: need {} more", s),
    }
}

//-----------------//
// Tests
//-----------------//
#[test]
fn test_basic_parsers() {
    assert_eq!(symbol("#hello"), Ok(("hello", '#')));
    assert_eq!(letter("hello"), Ok(("ello", 'h')));
    assert_eq!(digit("123"), Ok(("23", '1')));
    assert_eq!(spaces("   123"), Ok(("123", "   ")));
}

#[test]
fn test_lisp_atom() {
    assert_eq!(lisp_atom("#f"), Ok(("", LBool(false))));
    assert_eq!(lisp_atom("#t"), Ok(("", LBool(true))));
    assert_eq!(lisp_atom("variable"), Ok(("", LAtom("variable".to_string()))));
}

#[test]
fn test_lisp_number() {
    assert_eq!(lisp_number("123"), Ok(("", LNumber(123))));
}

#[test]
fn test_lisp_string() {
    assert_eq!(lisp_string("\"hello\""), Ok(("", LString("hello".to_string()))));
}
