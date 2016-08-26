use std::{io, process};
use nodes::{ParagraphVec, Paragraph, ParagraphLine, ParagraphWord};

pub fn run(args: &Vec<String>) -> io::Result<ParagraphVec> {
	let output = try!( process::Command::new("mecab").args(&args).output() );
	let text = String::from_utf8_lossy(&output.stdout);

	return Ok(parse(&text));
}

pub fn parse(text: &str) -> ParagraphVec {
	let mut paragraphs: ParagraphVec = ParagraphVec::new();
	let mut paragraph: Paragraph = Paragraph::new();
	let mut line: ParagraphLine = ParagraphLine::new();
	let mut word: ParagraphWord = ParagraphWord::new();
	let mut eos_count = 0;

	for input in text.lines() {
		let mut parts = input.split("\t");
		let original;

		if let Some(s) = parts.next() {
			original = s;
		} else {
			continue;
		}

		if original == "EOS" {
			eos_count += 1;
			continue;
		}

		if eos_count > 0 && !line.is_empty() {
			paragraph.push(line);
			line = ParagraphLine::new();
		}

		if eos_count > 1 && !paragraph.is_empty() {
			paragraphs.push(paragraph);
			paragraph = Paragraph::new();
		}

		word.original.push_str(original);
		if let Some(kana) = parts.next() {
			word.kana.push_str(kana);
		}

		if !word.is_empty() && !original.ends_with("っ") {
			line.push(word);
			word = ParagraphWord::new();
		}

		eos_count = 0;
	}

	if !word.is_empty() {
		line.push(word);
	}

	if !line.is_empty() {
		paragraph.push(line);
	}

	if !paragraph.is_empty() {
		paragraphs.push(paragraph);
	}

	return paragraphs;
}
