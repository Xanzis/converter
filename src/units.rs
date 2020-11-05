extern crate derive_more;
use derive_more::{Add, Mul};

use std::ops::Mul;
use std::ops::Add;
use std::fmt::Debug;

#[derive(Debug)]
pub struct SiValue {
	real: f64,
	unit: SiUnit,
}

#[derive(Add, Mul, PartialEq, Debug)]
pub struct SiUnit {
	meter: i64,
	second: i64,
	mole: i64,
	ampere: i64,
	kelvin: i64,
	candela: i64,
	kilogram: i64,
}

impl SiValue {
	pub fn pow(self, i: i64) -> SiValue {
		SiValue {
			real: self.real.powi(i as i32),
			unit: self.unit * i,
		}
	}
}

impl Mul<f64> for SiValue {
	type Output = SiValue;
	fn mul(self, rhs: f64) -> Self::Output {
		SiValue {
			real: self.real * rhs,
			unit: self.unit,
		}
	}
}

impl Mul<SiValue> for SiValue {
	type Output = SiValue;
	fn mul(self, rhs: SiValue) -> Self::Output {
		SiValue {
			real: self.real * rhs.real,
			unit: self.unit + rhs.unit,
		}
	}
}

impl Add<SiValue> for SiValue {
	type Output = SiValue;
	fn add(self, rhs: SiValue) -> Self::Output {
		if self.unit == rhs.unit {
			return SiValue {
				real: self.real + rhs.real,
				unit: self.unit,
			}
		}
		else {
			panic!("cannot add {:?} and {:?}", self, rhs);
		}
	}
}

impl Default for SiUnit {
	fn default() -> SiUnit {
		SiUnit {
			meter: 0,
			second: 0,
			mole: 0,
			ampere: 0,
			kelvin: 0,
			candela: 0,
			kilogram: 0,
		}
	}
}

impl From<String> for SiValue {
	fn from(unit_name: String) -> Self {
		// match unit names to SI values
		match unit_name.as_str() {
			"m" => SiValue {
				real: 1.0,
				unit: SiUnit {meter: 1, ..Default::default()},
			},
			_ => { panic!("uninplemented unit name"); },
		}
	}
}

impl From<f64> for SiValue {
	fn from(scalar: f64) -> Self {
		SiValue {
			real: scalar,
			unit: SiUnit {..Default::default()},
		}
	}
}