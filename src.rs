mod testy {
	pub struct Testy {
		pub field: String,
	}
}

/// Should resolve to field [testy::Testy::field] `test`
fn hello() {
	todo!();
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
	hello();

	return Ok(());
}
