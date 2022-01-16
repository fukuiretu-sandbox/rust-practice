use std::fmt::Debug;

// [x] 整数閉区間を示すクラス(や構造体)を作りたい。
// [x] 整数閉区間は下端点と上端点を持ち、整数閉区間の文字列表記を返せる
// [x] (例： 下端点3、上端点7の整数閉区間の文字列表記は"[3, 7]")
// [x] また、整数閉区間は指定した整数を含むかどうかを判定できる。

#[derive(Debug)]
struct IntClosedRange {
    lower: i32,
    upper: i32,
}

impl IntClosedRange {
    fn new(lower: i32, upper: i32) -> Self {
        Self {
            lower: lower,
            upper: upper,
        }
    }

    fn noation(&self) -> String {
        // "[]".to_string()
        format!("[{}, {}]", self.lower, self.upper)
    }

    fn includes(&self, arg: i32) -> Result<usize, &'static str> {
        if self.lower <= arg && self.upper >= arg {
            Ok(1)
        } else {
            Err("Failiure")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::IntClosedRange;

    #[test]
    fn test_lower() {
        let int_closed_range = IntClosedRange::new(3, 7);
        assert_eq!(3, int_closed_range.lower);
    }

    #[test]
    fn test_upper() {
        let int_closed_range = IntClosedRange::new(3, 7);
        assert_eq!(7, int_closed_range.upper);
    }

    #[test]
    fn test_noation() {
        let int_closed_range = IntClosedRange::new(3, 7);
        assert_eq!("[3, 7]", int_closed_range.noation());
    }

    #[test]
    fn test_includes_1() {
        let int_closed_range = IntClosedRange::new(3, 7);
        assert_eq!(Ok(1), int_closed_range.includes(3));
    }

    #[test]
    fn test_includes_3() {
        let int_closed_range = IntClosedRange::new(3, 7);
        assert_eq!(Ok(1), int_closed_range.includes(7));
    }

    #[test]
    fn test_includes_2() {
        let int_closed_range = IntClosedRange::new(3, 7);
        assert_eq!(Err("Failiure"), int_closed_range.includes(8));
    }
}
