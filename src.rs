
fn main() -> Result<(), Box<dyn std::error::Error>> {
	use crossbeam::channel::*;

	let (tx1, rx1) = unbounded::<usize>();
	let (tx2, rx2) = unbounded::<usize>();

	// if the following is just *one* "recv", completion works until something invalid
	select! {
		recv(rx1) -> msg => {
			// no completion at all
		},
		recv(rx2) -> msg => {
			// no completion at all
		},
		default => (),
	}

	return Ok(());
}
