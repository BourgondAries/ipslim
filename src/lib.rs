#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

use std::time::{Duration, Instant};
use std::thread::sleep;

pub struct Lim {
	last: Instant,
	npi: u64,
}

fn ips2npi(ips: u64) -> u64 {
	if ips > 0 {
		(1000.0*1000.0*1000.0 / ips as f64) as u64
	} else {
		0
	}
}

impl Lim {
	pub fn new(ips: u64) -> Lim {
		Lim {
			last: Instant::now(),
			npi: ips2npi(ips),
		}
	}

	pub fn limit(&mut self) {
		let left = Instant::now() - self.last;
		let nanoleft = left.as_secs() * 1000*1000*1000 + left.subsec_nanos() as u64;
		if self.npi > nanoleft {
			let sleeptime = self.npi - nanoleft;
			let sleeptime = Duration::new(sleeptime / 1000000000, (sleeptime % 1000000000) as u32);
			sleep(sleeptime);
		}
		self.last = Instant::now();
	}
}

#[cfg(test)]
mod tests {
	use super::Lim;
	use std::time::Instant;

	#[test]
	fn basic() {
		let mut lim = Lim::new(15);
		let begin = Instant::now();
		for _ in 0..60 {
			lim.limit();
		}
		let end = Instant::now();
		let elaps = end - begin;
		assert!(elaps.as_secs() >= 3);
		assert!(elaps.as_secs() <= 5);
	}

	#[test]
	fn edge() {
		let mut lim = Lim::new(0);
		let begin = Instant::now();
		for _ in 0..60 {
			lim.limit();
		}
		let end = Instant::now();
		let elaps = end - begin;
		assert!(elaps.as_secs() <= 0);
	}

}
