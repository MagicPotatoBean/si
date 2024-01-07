#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declare_si_value() {
        let x = SiValue::new(1, 2, 3, 4, 5, 6, 7, 8);
        assert_eq!(
            x,
            SiValue {
                value: Some(1),
                unit: Some(SiUnit {
                    length: 2,
                    mass: 3,
                    time: 4,
                    temperature: 5,
                    current: 6,
                    amount: 7,
                    luminous_intensity: 8
                }),
            }
        )
    }
    #[test]
    fn add_i32_si_values() {
        let x = SiValue::new(1, 2, 3, 4, 5, 6, 7, 8);
        let y = SiValue::new(10, 2, 3, 4, 5, 6, 7, 8);
        let z = SiValue::new(11, 2, 3, 4, 5, 6, 7, 8);
        assert_eq!(x + y, z);
        assert_eq!(y + x, z);
        let y = SiValue::new(10, 20, 30, 40, 50, 60, 70, 80);
        let z = SiValue::new(11, 22, 33, 44, 55, 66, 77, 88);
        assert_eq!(x + y, SiValue::default());
        assert_eq!(x + z, SiValue::default());
    }
    #[test]
    fn sub_i32_si_values() {
        let x = SiValue::new(1, 2, 3, 4, 5, 6, 7, 8);
        let y = SiValue::new(10, 2, 3, 4, 5, 6, 7, 8);
        let z = SiValue::new(9, 2, 3, 4, 5, 6, 7, 8);
        assert_eq!(y - x, z);
        assert_eq!(y - z, x);
    }
    #[test]
    fn mul_i32_si_values() {
        let x = SiValue::new(5, 2, 3, 4, 5, 6, 7, 8);
        let y = SiValue::new(10, 20, 30, 40, 50, 60, 70, 80);
        let z = SiValue::new(50, 22, 33, 44, 55, 66, 77, 88);
        assert_eq!(y * x, z);
        assert_eq!(x * y, z);
    }
    #[test]
    fn div_i32_si_values() {
        let x = SiValue::new(5, 2, 3, 4, 5, 6, 7, 8);
        let y = SiValue::new(10, 20, 30, 40, 50, 60, 70, 80);
        let z = SiValue::new(50, 22, 33, 44, 55, 66, 77, 88);
        assert_eq!(z / y, x);
        assert_eq!(z / x, y);
    }
    #[test]
    fn div_f64_si_values() {
        let x = SiValue::new(1, 2, 3, 4, 5, 6, 7, 8);
        let y = SiValue::new(10, 20, 30, 40, 50, 60, 70, 80);
        let z = SiValue::new(0.1, -18, -27, -36, -45, -54, -63, -72);
        assert_eq!(x.si_into::<f64, i32>() / y.si_into::<f64, i32>(), z);
        assert_eq!(x.si_into() / z, y.si_into());
    }
    #[test]
    fn display_unit_test() {
        assert_eq!(
            format!("{}", SiUnit::new(2, 3, 4, 5, 6, 7, 8)),
            "(m^2)(kg^3)(s^4)(k^5)(A^6)(mol^7)(cd^8)"
        );
    }
    #[test]
    fn display_value_test() {
        assert_eq!(
            format!("{}", SiValue::new(1, 2, 3, 4, 5, 6, 7, 8)),
            "1 (m^2)(kg^3)(s^4)(k^5)(A^6)(mol^7)(cd^8)"
        );
    }
}
use std::{fmt::Display, ops::{Add, Div, Mul, Sub}};
#[derive(Clone, Debug, Copy)]
pub struct SiValue<T, U> {
    value: Option<T>,
    unit: Option<SiUnit<U>>,
}
impl<T: PartialEq, U: PartialEq> PartialEq for SiValue<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.unit == other.unit
    }
}
impl<T: Add<Output = T>, U: PartialEq> Add<SiValue<T, U>> for SiValue<T, U> {
    type Output = SiValue<T, U>;

    fn add(self, rhs: SiValue<T, U>) -> Self::Output {
        if let ((Some(lhs_val), Some(lhs_unit)), (Some(rhs_val), Some(rhs_unit))) =
            ((self.value, self.unit), (rhs.value, rhs.unit))
        {
            if lhs_unit == rhs_unit {
                SiValue {
                    value: Some(lhs_val + rhs_val),
                    unit: Some(lhs_unit),
                }
            } else {
                SiValue::default()
            }
        } else {
            SiValue::default()
        }
    }
}
impl<T: Sub<Output = T>, U: PartialEq> Sub<SiValue<T, U>> for SiValue<T, U> {
    type Output = SiValue<T, U>;

    fn sub(self, rhs: SiValue<T, U>) -> Self::Output {
        if let ((Some(lhs_val), Some(lhs_unit)), (Some(rhs_val), Some(rhs_unit))) =
            ((self.value, self.unit), (rhs.value, rhs.unit))
        {
            if lhs_unit == rhs_unit {
                SiValue {
                    value: Some(lhs_val - rhs_val),
                    unit: Some(lhs_unit),
                }
            } else {
                SiValue::default()
            }
        } else {
            SiValue::default()
        }
    }
}
impl<T: Mul<Output = T>, U: Add<Output = U>> Mul for SiValue<T, U> {
    type Output = SiValue<T, U>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut ret_val: SiValue<T, U> = SiValue::default();
        if let ((Some(lhs_val), Some(lhs_unit)), (Some(rhs_val), Some(rhs_unit))) =
            ((self.value, self.unit), (rhs.value, rhs.unit))
        {
            ret_val.unit = Some(lhs_unit * rhs_unit);
            ret_val.value = Some(lhs_val * rhs_val);
        } else {
            ret_val = SiValue::default();
        }
        ret_val
    }
}
impl<T: Div<Output = T>, U: Sub<Output = U>> Div for SiValue<T, U> {
    type Output = SiValue<T, U>;

    fn div(self, rhs: Self) -> Self::Output {
        let mut ret_val: SiValue<T, U> = SiValue::default();
        if let ((Some(lhs_val), Some(lhs_unit)), (Some(rhs_val), Some(rhs_unit))) =
            ((self.value, self.unit), (rhs.value, rhs.unit))
        {
            ret_val.unit = Some(lhs_unit / rhs_unit);
            ret_val.value = Some(lhs_val / rhs_val);
        } else {
            ret_val = SiValue::default();
        }
        ret_val
    }
}
impl<T, U> Default for SiValue<T, U> {
    fn default() -> Self {
        SiValue {
            value: None,
            unit: None,
        }
    }
}
impl<T: Display + From<u8> + PartialEq, U: Display + From<u8> + PartialEq> Display for SiValue<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let (Some(val), Some(unit)) = (self.value.as_ref(), self.unit.as_ref()) {
            let mut display_val = String::default();
            if *val != T::from(0u8) {
                display_val.push_str(format!("{}", val).as_str());
            }
            write!(f, "{} {}", display_val.as_str(), unit)
        } else {
            write!(f, "Value is None")
        }
    }
}
impl<T, U> SiValue<T, U> {
    pub fn si_into<V, W>(self) -> SiValue<V, W>
    where
        T: Into<V>,
        U: Into<W>,
    {
        SiValue {
            unit: if let Some(unit_val) = self.unit {
                Some(SiUnit {
                    length: unit_val.length.into(),
                    mass: unit_val.mass.into(),
                    time: unit_val.time.into(),
                    temperature: unit_val.temperature.into(),
                    current: unit_val.current.into(),
                    amount: unit_val.amount.into(),
                    luminous_intensity: unit_val.luminous_intensity.into(),
                })
            } else {
                None
            },
            value: self.value.map(|x| x.into()),
        }
    }
    pub fn new(value: T, length: U, mass: U, time: U, temperature: U, current: U, amount: U, luminous_intensity: U) -> SiValue<T, U> {
        SiValue {
            value: Some(value),
            unit: Some(SiUnit {
                length,
                mass,
                time,
                temperature,
                current,
                amount,
                luminous_intensity,
            }),
        }
    }
}
#[derive(Clone, Debug, Copy)]
struct SiUnit<T> {
    length: T,
    mass: T,
    time: T,
    temperature: T,
    current: T,
    amount: T,
    luminous_intensity: T,
}
impl<T: PartialEq> PartialEq for SiUnit<T> {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length
            && self.mass == other.mass
            && self.time == other.time
            && self.temperature == other.temperature
            && self.current == other.current
            && self.amount == other.amount
            && self.luminous_intensity == other.luminous_intensity
    }
}
impl<T: Add<Output = T>> Mul for SiUnit<T> {
    type Output = SiUnit<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        SiUnit {
            length: rhs.length + self.length,
            mass: rhs.mass + self.mass,
            time: rhs.time + self.time,
            temperature: rhs.temperature + self.temperature,
            current: rhs.current + self.current,
            amount: rhs.amount + self.amount,
            luminous_intensity: rhs.luminous_intensity + self.luminous_intensity,
        }
    }
}
impl<T: Sub<Output = T>> Div for SiUnit<T> {
    type Output = SiUnit<T>;

    fn div(self, rhs: Self) -> Self::Output {
        SiUnit {
            length: self.length - rhs.length,
            mass: self.mass - rhs.mass,
            time: self.time - rhs.time,
            temperature: self.temperature - rhs.temperature,
            current: self.current - rhs.current,
            amount: self.amount - rhs.amount,
            luminous_intensity: self.luminous_intensity - rhs.luminous_intensity,
        }
    }
}
impl<T: Display + From<u8> + PartialEq> Display for SiUnit<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display_val = String::default();
        if self.length != T::from(0u8) {
            display_val.push_str(format!("(m^{})", self.length).as_str());
        }
        if self.mass != T::from(0u8) {
            display_val.push_str(format!("(kg^{})", self.mass).as_str());
        }
        if self.time != T::from(0u8) {
            display_val.push_str(format!("(s^{})", self.time).as_str());
        }
        if self.temperature != T::from(0u8) {
            display_val.push_str(format!("(k^{})", self.temperature).as_str());
        }
        if self.current != T::from(0u8) {
            display_val.push_str(format!("(A^{})", self.current).as_str());
        }
        if self.amount != T::from(0u8) {
            display_val.push_str(format!("(mol^{})", self.amount).as_str());
        }
        if self.luminous_intensity != T::from(0u8) {
            display_val.push_str(format!("(cd^{})", self.luminous_intensity).as_str());
        }
        write!(f, "{}", display_val.as_str())
    }
}
impl<T> SiUnit<T> {
    pub fn new(length: T, mass: T, time: T, temperature: T, current: T, amount: T, luminous_intensity: T) -> SiUnit<T> {
        SiUnit {
            length,
            mass,
            time,
            temperature,
            current,
            amount,
            luminous_intensity,
        }
    }
}