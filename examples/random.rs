use genpdf::{elements, fonts, style, Element as _};

fn main() {
	let par = elements::Paragraph::new("Hello world!")
		.styled_string("test Bold", style::Style::new().bold())
		.styled_string("test Intalic", style::Style::new().italic())
		.string("test123");

	
	println!("Test: {}", par);

	let txt = elements::Text::new("Test 2");

	println!("Test2: {}", txt);
}