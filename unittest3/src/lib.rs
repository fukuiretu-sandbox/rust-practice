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

    fn includes(&self, arg: i32) -> bool {
        if self.lower <= arg && self.upper >= arg {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate rstest;
    extern crate speculate;

    use crate::IntClosedRange;
    use rstest::*;
    use speculate::speculate;

    speculate! {
        describe "IntClosedRangeは整数閉区間を表す" {
            describe "IntClosedRangeは下端点と上端点を持つ" {
                it "下端点は3を返す" {
                    #[rstest]
                    fn lower() {
                        let range = IntClosedRange::new(3, 7);
                        assert_eq!(3, range.lower);
                    }
                }
                it "上端点は7を返す" {
                    #[rstest]
                    fn upper() {
                        let range = IntClosedRange::new(3, 7);
                        assert_eq!(7, range.upper);
                    }
                }
            }
            describe "IntClosedRangeは整数閉区間の文字列表記を返す" {
                #[rstest]
                fn noation() {
                    let range = IntClosedRange::new(3, 7);
                    assert_eq!("[3, 7]", range.noation());
                }
            }
            describe "IntClosedRangeは指定した整数を含むか判定できる" {
                it "下端点の境界値を含む" {
                    #[rstest]
                    fn includes_true() {
                        let range = IntClosedRange::new(3, 7);
                        assert_eq!(true, range.includes(3))
                    }
                }

                it "下端点の境界値を含まない" {
                    #[rstest]
                    fn includes_true() {
                        let range = IntClosedRange::new(3, 7);
                        assert_eq!(false, range.includes(2))
                    }
                }

                it "上端点の境界値を含む" {
                    #[rstest]
                    fn includes_true() {
                        let range = IntClosedRange::new(3, 7);
                        assert_eq!(true, range.includes(7))
                    }
                }

                it "上端点の境界値を含まない" {
                    #[rstest]
                    fn includes_true() {
                        let range = IntClosedRange::new(3, 7);
                        assert_eq!(false, range.includes(8))
                    }
                }
            }
        }
    }
}
