extern crate derive_more;
use derive_more::{Add, Mul};

use std::ops::Mul;
use std::ops::Add;
use std::fmt::{self, Debug};
use std::error;

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

#[derive(Clone, Debug)]
pub struct UnitError {
	message: String,
}

impl UnitError {
	fn new(x: &str) -> UnitError {
		UnitError {message: format!("unit_error: {}", String::from(x))}
	}
}

impl fmt::Display for UnitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for UnitError {}

impl SiValue {
	pub fn pow(self, i: i64) -> SiValue {
		SiValue {
			real: self.real.powi(i as i32),
			unit: self.unit * i,
		}
	}

	pub fn try_from(name: &str) -> Result<SiValue, UnitError> {
		let res =  SiValue::from(name);
		if res.real < 0.0 { return Err(UnitError::new(format!("unimplemented unit: {}", name).as_str())) }
		Ok(res)
	}
}

impl fmt::Display for SiValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.real)?;
        let pows = vec![self.unit.meter, self.unit.second, self.unit.mole, self.unit.ampere, 
        	self.unit.kelvin, self.unit.candela, self.unit.kilogram];
        let names = vec!["m", "s", "mol", "A", "K", "cd", "kg"];
        for (i, p) in pows.iter().enumerate() {
        	if *p != 0 {
        		write!(f, " {}{}", names[i], p)?
        	}
        }
        Ok(())
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
	type Output = Result<SiValue, UnitError>;
	fn add(self, rhs: SiValue) -> Self::Output {
		if self.unit == rhs.unit {
			return Ok(SiValue {
				real: self.real + rhs.real,
				unit: self.unit,
			})
		}
		else {
			return Err(UnitError::new(format!("cannot add {:?} and {:?}", self, rhs).as_str()));
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

impl From<&str> for SiValue {
	fn from(unit_name: &str) -> Self {
		// match unit names to SI values
		match unit_name {
			// base SI units
			"m" => SiValue {
				real: 1.0,
				unit: SiUnit {meter: 1, ..Default::default()},
			},
			"s" => SiValue {
				real: 1.0,
				unit: SiUnit {second: 1, ..Default::default()},
			},
			"mol" => SiValue {
				real: 1.0,
				unit: SiUnit {mole: 1, ..Default::default()},
			},
			"A" => SiValue {
				real: 1.0,
				unit: SiUnit {ampere: 1, ..Default::default()},
			},
			"K" => SiValue {
				real: 1.0,
				unit: SiUnit {kelvin: 1, ..Default::default()},
			},
			"cd" => SiValue {
				real: 1.0,
				unit: SiUnit {candela: 1, ..Default::default()},
			},
			"kg" => SiValue {
				real: 1.0,
				unit: SiUnit {kilogram: 1, ..Default::default()},
			},
			// standard SI variants
			"Pa" => SiValue::from("N") * SiValue::from("m").pow(-2),
			"N" => SiValue::from("kg") * SiValue::from("m") * SiValue::from("s").pow(-2),
			"J" => SiValue::from("N") * SiValue::from("m"),
			"W" => SiValue::from("J") * SiValue::from("s").pow(-1),
			"kW" => SiValue::from("W") * 1000.0,
			"MW" => SiValue::from("W") * 1_000_000.0,
			"km" => SiValue::from("m") * 1000.0,
			"cm" => SiValue::from("m") * 0.01,
			"mm" => SiValue::from("m") * 0.001,
			"mL" => SiValue::from("cm").pow(3),
			"L" => SiValue::from("mL") * 1000.0,
			// common time measurements
			"ms" => SiValue::from("s") * 0.001,
			"us" => SiValue::from("ms") * 0.001,
			"hr" => SiValue::from("s") * 3600.0,
			"day" => SiValue::from("hr") * 24.0,
			"yr" => SiValue::from("day") * 365.25,
			// common imperial units
			"ft" => SiValue::from("m") * 0.3048,
			"lbf" => SiValue::from("N") * 4.448,
			"in" => SiValue::from("m") * 0.0254,
			"thou" => SiValue::from("in") * 0.001,
			"mile" => SiValue::from("m") * 1609.0,
			"mph" => SiValue::from("m") * SiValue::from("s").pow(-1) * 0.447,
			_ => SiValue {
				real: -1.0,
				unit: SiUnit {..Default::default()},
			}
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