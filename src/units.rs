struct SiUnit {
	meter: i64,
	second: i64,
	mole: i64,
	ampere: i64,
	kelvin: i64,
	candela: i64,
	kilogram: i64,
}

struct Quantity {
	value: f64,
	unit: SiUnit,
}