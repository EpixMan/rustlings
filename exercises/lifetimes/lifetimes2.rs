// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let oo = "long string is long";
    let string1: &str = &String::from(oo);
    let string2_slot;

    let result: &str=
        {
            string2_slot = String::from("xyz");
            longest(string1, &string2_slot)
        };
    println!("The longest string is '{}'", result);
}
