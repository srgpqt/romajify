#[macro_use]
extern crate lazy_static;
extern crate regex;

mod mecab;
mod nodes;
mod html;

fn main() {
	let args = std::env::args().skip(1).collect::<Vec<String>>();

	let parsed = mecab::run(&args)
		.expect("failed to execute `mecab' subprocess");

	let html = html::generate(&parsed);

	println!("{}", html);
}
