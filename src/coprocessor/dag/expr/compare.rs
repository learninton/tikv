// Copyright 2017 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

use std::i64;
use std::cmp::Ordering;

use tipb::expression::ScalarFuncSig;

use coprocessor::codec::{mysql, Datum};
use super::{StatementContext, Result, FnCall};

impl FnCall {
    pub fn compare_int(&self,
                       ctx: &StatementContext,
                       row: &[Datum],
                       sig: ScalarFuncSig)
                       -> Result<Option<i64>> {
        let lhs = try!(self.children[0].eval_int(ctx, row));
        let rhs = try!(self.children[1].eval_int(ctx, row));
        match (lhs, rhs) {
            (None, _) | (_, None) => {
                if sig == ScalarFuncSig::NullEQInt {
                    Ok(Some(1))
                } else {
                    Ok(None)
                }
            }
            (Some(lhs), Some(rhs)) => {
                let lhs_unsigned = mysql::has_unsigned_flag(self.children[0].get_tp().get_flag());
                let rhs_unsigned = mysql::has_unsigned_flag(self.children[1].get_tp().get_flag());
                let ordering = cmp_i64_with_unsigned_flag(lhs, lhs_unsigned, rhs, rhs_unsigned);
                Ok(Some(match sig {
                    ScalarFuncSig::LTInt => ordering == Ordering::Less,
                    ScalarFuncSig::LEInt => ordering != Ordering::Greater,
                    ScalarFuncSig::GTInt => ordering == Ordering::Greater,
                    ScalarFuncSig::GEInt => ordering != Ordering::Less,
                    ScalarFuncSig::EQInt => ordering == Ordering::Equal,
                    ScalarFuncSig::NEInt => ordering != Ordering::Equal,
                    _ => unreachable!(),
                } as i64))
            }
        }
    }

    pub fn compare_real(&self,
                        ctx: &StatementContext,
                        row: &[Datum],
                        sig: ScalarFuncSig)
                        -> Result<Option<i64>> {
        let lhs = try!(self.children[0].eval_real(ctx, row));
        let rhs = try!(self.children[1].eval_real(ctx, row));
        match (lhs, rhs) {
            (None, _) | (_, None) => {
                if sig == ScalarFuncSig::NullEQReal {
                    Ok(Some(1))
                } else {
                    Ok(None)
                }
            }
            (Some(lhs), Some(rhs)) => {
                let ordering = cmp_f64(lhs, rhs);
                Ok(Some(match sig {
                    ScalarFuncSig::LTReal => ordering == Ordering::Less,
                    ScalarFuncSig::LEReal => ordering != Ordering::Greater,
                    ScalarFuncSig::GTReal => ordering == Ordering::Greater,
                    ScalarFuncSig::GEReal => ordering != Ordering::Less,
                    ScalarFuncSig::EQReal => ordering == Ordering::Equal,
                    ScalarFuncSig::NEReal => ordering != Ordering::Equal,
                    _ => unreachable!(),
                } as i64))
            }
        }
    }

    pub fn compare_decimal(&self,
                           ctx: &StatementContext,
                           row: &[Datum],
                           sig: ScalarFuncSig)
                           -> Result<Option<i64>> {
        let lhs = try!(self.children[0].eval_decimal(ctx, row));
        let rhs = try!(self.children[1].eval_decimal(ctx, row));
        match (lhs, rhs) {
            (None, _) | (_, None) => {
                if sig == ScalarFuncSig::NullEQDecimal {
                    Ok(Some(1))
                } else {
                    Ok(None)
                }
            }
            (Some(lhs), Some(rhs)) => {
                let ordering = lhs.cmp(&rhs);
                Ok(Some(match sig {
                    ScalarFuncSig::LTDecimal => ordering == Ordering::Less,
                    ScalarFuncSig::LEDecimal => ordering != Ordering::Greater,
                    ScalarFuncSig::GTDecimal => ordering == Ordering::Greater,
                    ScalarFuncSig::GEDecimal => ordering != Ordering::Less,
                    ScalarFuncSig::EQDecimal => ordering == Ordering::Equal,
                    ScalarFuncSig::NEDecimal => ordering != Ordering::Equal,
                    _ => unreachable!(),
                } as i64))
            }
        }
    }

    pub fn compare_string(&self,
                          ctx: &StatementContext,
                          row: &[Datum],
                          sig: ScalarFuncSig)
                          -> Result<Option<i64>> {
        let lhs = try!(self.children[0].eval_string(ctx, row));
        let rhs = try!(self.children[1].eval_string(ctx, row));
        match (lhs, rhs) {
            (None, _) | (_, None) => {
                if sig == ScalarFuncSig::NullEQString {
                    Ok(Some(1))
                } else {
                    Ok(None)
                }
            }
            (Some(lhs), Some(rhs)) => {
                let ordering = lhs.cmp(&rhs);
                Ok(Some(match sig {
                    ScalarFuncSig::LTString => ordering == Ordering::Less,
                    ScalarFuncSig::LEString => ordering != Ordering::Greater,
                    ScalarFuncSig::GTString => ordering == Ordering::Greater,
                    ScalarFuncSig::GEString => ordering != Ordering::Less,
                    ScalarFuncSig::EQString => ordering == Ordering::Equal,
                    ScalarFuncSig::NEString => ordering != Ordering::Equal,
                    _ => unreachable!(),
                } as i64))
            }
        }
    }

    pub fn compare_time(&self,
                        ctx: &StatementContext,
                        row: &[Datum],
                        sig: ScalarFuncSig)
                        -> Result<Option<i64>> {
        let lhs = try!(self.children[0].eval_time(ctx, row));
        let rhs = try!(self.children[1].eval_time(ctx, row));
        match (lhs, rhs) {
            (None, _) | (_, None) => {
                if sig == ScalarFuncSig::NullEQTime {
                    Ok(Some(1))
                } else {
                    Ok(None)
                }
            }
            (Some(lhs), Some(rhs)) => {
                let ordering = lhs.cmp(&rhs);
                Ok(Some(match sig {
                    ScalarFuncSig::LTTime => ordering == Ordering::Less,
                    ScalarFuncSig::LETime => ordering != Ordering::Greater,
                    ScalarFuncSig::GTTime => ordering == Ordering::Greater,
                    ScalarFuncSig::GETime => ordering != Ordering::Less,
                    ScalarFuncSig::EQTime => ordering == Ordering::Equal,
                    ScalarFuncSig::NETime => ordering != Ordering::Equal,
                    _ => unreachable!(),
                } as i64))
            }
        }
    }

    pub fn compare_duration(&self,
                            ctx: &StatementContext,
                            row: &[Datum],
                            sig: ScalarFuncSig)
                            -> Result<Option<i64>> {
        let lhs = try!(self.children[0].eval_duration(ctx, row));
        let rhs = try!(self.children[1].eval_duration(ctx, row));
        match (lhs, rhs) {
            (None, _) | (_, None) => {
                if sig == ScalarFuncSig::NullEQDuration {
                    Ok(Some(1))
                } else {
                    Ok(None)
                }
            }
            (Some(lhs), Some(rhs)) => {
                let ordering = lhs.cmp(&rhs);
                Ok(Some(match sig {
                    ScalarFuncSig::LTDuration => ordering == Ordering::Less,
                    ScalarFuncSig::LEDuration => ordering != Ordering::Greater,
                    ScalarFuncSig::GTDuration => ordering == Ordering::Greater,
                    ScalarFuncSig::GEDuration => ordering != Ordering::Less,
                    ScalarFuncSig::EQDuration => ordering == Ordering::Equal,
                    ScalarFuncSig::NEDuration => ordering != Ordering::Equal,
                    _ => unreachable!(),
                } as i64))
            }
        }
    }

    pub fn compare_json(&self,
                        ctx: &StatementContext,
                        row: &[Datum],
                        sig: ScalarFuncSig)
                        -> Result<Option<i64>> {
        let lhs = try!(self.children[0].eval_json(ctx, row));
        let rhs = try!(self.children[1].eval_json(ctx, row));
        match (lhs, rhs) {
            (None, _) | (_, None) => {
                if sig == ScalarFuncSig::NullEQJson {
                    Ok(Some(1))
                } else {
                    Ok(None)
                }
            }
            (Some(lhs), Some(rhs)) => {
                let ordering = lhs.cmp(&rhs);
                Ok(Some(match sig {
                    ScalarFuncSig::LTJson => ordering == Ordering::Less,
                    ScalarFuncSig::LEJson => ordering != Ordering::Greater,
                    ScalarFuncSig::GTJson => ordering == Ordering::Greater,
                    ScalarFuncSig::GEJson => ordering != Ordering::Less,
                    ScalarFuncSig::EQJson => ordering == Ordering::Equal,
                    ScalarFuncSig::NEJson => ordering != Ordering::Equal,
                    _ => unreachable!(),
                } as i64))
            }
        }
    }
}

#[inline]
fn cmp_i64_with_unsigned_flag(lhs: i64,
                              lhs_unsigned: bool,
                              rhs: i64,
                              rhs_unsigned: bool)
                              -> Ordering {
    match (lhs_unsigned, rhs_unsigned) {
        (true, true) | (false, false) => lhs.cmp(&rhs),
        (true, false) => {
            if rhs < 0 || lhs as u64 > i64::MAX as u64 {
                Ordering::Greater
            } else {
                lhs.cmp(&rhs)
            }
        }
        (false, true) => {
            if lhs < 0 || rhs as u64 > i64::MAX as u64 {
                Ordering::Less
            } else {
                lhs.cmp(&rhs)
            }
        }
    }
}

#[inline]
fn cmp_f64(lhs: f64, rhs: f64) -> Ordering {
    if lhs == rhs {
        Ordering::Equal
    } else if lhs < rhs {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

#[cfg(test)]
mod test {
    use std::{i64, u64};
    use super::*;

    #[test]
    fn test_cmp_i64_with_unsigned_flag() {
        let cases = vec![
            (5, false, 3, false, Ordering::Greater),
            (u64::MAX as i64, true, (u64::MAX - 1) as i64, true, Ordering::Greater),
            (5, true, i64::MIN, false, Ordering::Greater),
            (u64::MAX as i64, true, i64::MIN, false, Ordering::Greater),
            (5, true, 3, false, Ordering::Greater),
            (i64::MIN, false, 3, true, Ordering::Less),
            (5, false, u64::MAX as i64, true, Ordering::Less),
            (5, false, 3, true, Ordering::Greater),
        ];
        for (a, b, c, d, e) in cases {
            let o = cmp_i64_with_unsigned_flag(a, b, c, d);
            assert_eq!(o, e);
        }
    }
}
