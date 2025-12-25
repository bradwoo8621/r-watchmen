use crate::StdErr;

pub type StdR<T> = Result<T, StdErr>;
pub type VoidR = StdR<()>;

pub trait VoidResultHelper {
	fn collect(self, result: VoidR) -> Self;
	fn accumulate(self) -> VoidR;
}

impl VoidResultHelper for Vec<StdErr> {
	fn collect(mut self, result: VoidR) -> Self {
		if let Err(e) = result {
			self.push(e);
		}
		self
	}

	fn accumulate(mut self) -> VoidR {
		match self.len() {
			0 => Ok(()),
			1 => Err(self.remove(0)),
			_ => StdErr::accumulate(self),
		}
	}
}
