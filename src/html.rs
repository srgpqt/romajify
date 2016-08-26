use nodes::ParagraphVec;

static STYLE: &'static str = r###"
	body {
		background: #f7e8d0;
		color: #080800;
	}
	.paragraphs {
		display: flex;
		flex-direction: column;
	}
	.paragraph {
		flex: none;
		display: flex;
		flex-direction: column;
		margin: 0;
		padding: 1.5em 0;
		border-top: 1px dashed #cca;
	}
	.paragraph:first-child {
		border-top: none;
	}
	.line {
		flex: none;
		display: flex;
		flex-direction: row;
		padding: 0.5em 0;
	}
	.word {
		flex: none;
		display: flex;
		padding: 0.0625em 0.4em;
		flex-direction: column;
		border-left: 1px dashed #cca;
	}
	.word:first-child {
		border-left: none;
	}
	.original, .kana, .roma {
		flex: none;
		display: block;
		white-space: nowrap;
		text-align: center;
	}
	.original {
		font-size: 125%;
	}
	.kana, .roma {
		color: #997;
	}
	.kana {
		font-size: 75%;
	}
	.roma {
		margin-top: -0.5em;
		font-size: 75%;
	}
"###;

pub fn generate(paragraphs: &ParagraphVec) -> String {
	let mut content = String::with_capacity(2 * 1024 * 1024);

	content.push_str(&format!(r###"<!DOCTYPE html>
		<html>
		<head>
			<meta charset="utf-8" />
			<meta name="viewport" content="width=device-width, initial-scale=1" />
			<title>Romaji</title>

			<style>
				{style}
			</style>
		</head>
		<body>
	"###, style = STYLE));

	content.push_str("<div class=\"paragraphs\">");

	for paragraph in paragraphs {
		content.push_str("\n<p class=\"paragraph\">\n  ");
		
		for line in paragraph {
			content.push_str("<span class=\"line\">");

			for word in line {
				content.push_str("<span class=\"word\">");
					content.push_str("<span class=\"kana\">");
						if word.kana.is_empty() {
							content.push_str("&nbsp;");
						} else {
							content.push_str(escape_html(&word.kana).as_str());
						}
					content.push_str("</span>\n");

					content.push_str("    <span class=\"original\">");
						if word.original.is_empty() {
							content.push_str("&nbsp;");
						} else {
							content.push_str(escape_html(&word.original).as_str());
						}
					content.push_str("</span>\n");

					content.push_str("    <span class=\"roma\">");
						let roma = word.romanize();

						if roma.is_empty() {
							content.push_str("&nbsp;");
						} else {
							content.push_str(escape_html(&roma).as_str());
						}
					content.push_str("</span>\n");
				content.push_str("  </span>\n  ");
			}
			content.push_str("</span>\n  <br />");
		}
		content.push_str("\n</p>\n");
	}

	content.push_str("</div>");
	content.push_str("</body></html>");

	return content;
}

fn escape_html(text: &str) -> String {
	text.replace("\"", "&quot;")
		.replace("&", "&amp;")
		.replace("'", "&#x27;")
		.replace("<", "&lt;")
		.replace(">", "&gt;")
}
