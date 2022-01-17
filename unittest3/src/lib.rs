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
            #[fixture]
            fn fixture() -> IntClosedRange {
                IntClosedRange::new(3,7)
            }

            describe "IntClosedRangeは下端点と上端点を持つ" {
                it "下端点は3を返す" {
                    #[rstest]
                    fn lower(fixture: IntClosedRange) {
                        assert_eq!(3, fixture.lower);
                    }
                }
                it "上端点は7を返す" {
                    #[rstest]
                    fn upper(fixture: IntClosedRange) {
                        assert_eq!(7, fixture.upper);
                    }
                }
            }
            describe "IntClosedRangeは整数閉区間の文字列表記を返す" {
                #[rstest]
                fn noation(fixture: IntClosedRange) {
                    assert_eq!("[3, 7]", fixture.noation());
                }
            }
            describe "IntClosedRangeは指定した整数を含むか判定できる" {
                it "下端点の境界値を含む" {
                    #[rstest]
                    fn includes_true(fixture: IntClosedRange) {
                        assert_eq!(true, fixture.includes(3))
                    }
                }

                it "下端点の境界値を含まない" {
                    #[rstest]
                    fn includes_true(fixture: IntClosedRange) {
                        assert_eq!(false, fixture.includes(2))
                    }
                }

                it "上端点の境界値を含む" {
                    #[rstest]
                    fn includes_true(fixture: IntClosedRange) {
                        assert_eq!(true, fixture.includes(7))
                    }
                }

                it "上端点の境界値を含まない" {
                    #[rstest]
                    fn includes_true(fixture: IntClosedRange) {
                        assert_eq!(false, fixture.includes(8))
                    }
                }
            }
        }
    }
}
