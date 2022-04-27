use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    println!("Hello, world!");

    let x = action_values(&["first", "second"]);
    x.invoke(Context {});

    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}

fn action_values<'a>(vals: &[&str]) -> Action<'a> {
    action_callback(|_: Context| Action {
        rawValues: &[],
        callback: None,
    })
}

fn action_callback<'a>(f: Callback<'a>) -> Action<'a> {
    Action {
        rawValues: &[],
        callback: Option::Some(f),
    }
}

struct Context {}

struct RawValue<'a> {
    value: &'a str,
    description: &'a str,
}

type Callback<'a> = fn(c: Context) -> Action<'a>;

struct Action<'a> {
    rawValues: &'a [&'a RawValue<'a>],
    callback: Option<Callback<'a>>,
}

impl Action<'_> {
    fn invoke(&self, c: Context) -> Action<'_> {
        match self.callback {
            Some(f) => f(c),
            None => Action {
                rawValues: self.rawValues,
                callback: None,
            },
        }
    }
}
