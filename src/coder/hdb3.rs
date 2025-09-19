use crate::util;

use super::{LineCoder, SigElement};

#[derive(Debug, Clone, Copy)]
pub struct Hdb3 {
    tb: f64,
    v: f64,
}

impl Default for Hdb3 {
    fn default() -> Self {
        Self { tb: 1.0, v: 1.0 }
    }
}

impl Hdb3 {
    pub fn build(tb: f64, v: f64) -> anyhow::Result<Self> {
        Ok(Self {
            tb: util::check_bit_period(tb)?,
            v: util::check_ampl_opened(v)?,
        })
    }
}

#[derive(Clone, Copy)]
enum Symbol {
    Positive,
    Negative,
    Silence,
}

impl LineCoder for Hdb3 {
    fn encode(&self, bits: &[u8]) -> Box<[SigElement]> {
        let mut syms = Vec::new();
        let mut zero_count = 0;
        let mut marks = 0;
        let mut lm_pol = if self.v.is_sign_negative() {
            -self.v
        } else {
            self.v
        };

        for &bit in bits {
            if bit == 1 {
                lm_pol = -lm_pol;
                syms.push(if lm_pol > 0.0 {
                    Symbol::Positive
                } else {
                    Symbol::Negative
                });

                marks += 1;
                zero_count = 0;
                continue;
            }

            zero_count += 1;
            if zero_count == 4 {
                syms.truncate(syms.len().saturating_sub(zero_count - 1));
                if (marks & 1) != 0 {
                    syms.extend((0..(zero_count - 1)).map(|_| Symbol::Silence));
                    syms.push(if lm_pol > 0.0 {
                        Symbol::Positive
                    } else {
                        Symbol::Negative
                    });
                } else {
                    lm_pol = -lm_pol;
                    let pol_mark = if lm_pol > 0.0 {
                        Symbol::Positive
                    } else {
                        Symbol::Negative
                    };

                    syms.push(pol_mark);
                    syms.extend((0..(zero_count / 2)).map(|_| Symbol::Silence));
                    syms.push(pol_mark);
                }

                marks = 0;
                zero_count = 0;
            } else {
                syms.push(Symbol::Silence);
            }
        }

        let mut out = Vec::new();
        let mut t = 0.0;
        for sym in syms {
            let tf = t + self.tb;
            if tf > t {
                let lvl = match sym {
                    Symbol::Positive => self.v,
                    Symbol::Negative => -self.v,
                    Symbol::Silence => 0.0,
                };
                out.push(SigElement::new(t, tf, lvl));
            }
            t += self.tb;
        }

        out.into_boxed_slice()
    }

    fn on_tb(&mut self, tb: f64) -> anyhow::Result<()> {
        self.tb = util::check_bit_period(tb)?;
        Ok(())
    }

    fn on_v(&mut self, v: f64) -> anyhow::Result<()> {
        self.v = util::check_ampl_opened(v)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Hdb3;
    use crate::coder::{LineCoder, SigElement};

    #[test]
    fn test_hdb3_0000_even_to_b00v() {
        let seq = [0, 0, 0, 0];
        let expected = [
            SigElement::new(0.0, 1.0, 1.0),
            SigElement::new(1.0, 2.0, 0.0),
            SigElement::new(2.0, 3.0, 0.0),
            SigElement::new(3.0, 4.0, 1.0),
        ];

        let enc = Hdb3::build(1.0, -1.0).unwrap();
        assert_eq!(enc.encode(&seq).as_ref(), &expected);
    }

    #[test]
    fn test_hdb3_10000_odd_to_000v() {
        let seq = [1, 0, 0, 0, 0];
        let expected = [
            SigElement::new(0.0, 1.0, 1.0),
            SigElement::new(1.0, 2.0, 0.0),
            SigElement::new(2.0, 3.0, 0.0),
            SigElement::new(3.0, 4.0, 0.0),
            SigElement::new(4.0, 5.0, 1.0),
        ];

        let enc = Hdb3::build(1.0, -1.0).unwrap();
        assert_eq!(enc.encode(&seq).as_ref(), &expected);
    }

    #[test]
    fn test_hdb3_11000011() {
        let seq = [1, 1, 0, 0, 0, 0, 1, 1];
        let expected = [
            SigElement::new(0.0, 1.0, 1.0),
            SigElement::new(1.0, 2.0, -1.0),
            SigElement::new(2.0, 3.0, 1.0),
            SigElement::new(3.0, 4.0, 0.0),
            SigElement::new(4.0, 5.0, 0.0),
            SigElement::new(5.0, 6.0, 1.0),
            SigElement::new(6.0, 7.0, -1.0),
            SigElement::new(7.0, 8.0, 1.0),
        ];

        let enc = Hdb3::build(1.0, -1.0).unwrap();
        assert_eq!(enc.encode(&seq).as_ref(), &expected);
    }

    #[test]
    fn test_hdb3_00000000_two_blocks() {
        let seq = [0, 0, 0, 0, 0, 0, 0, 0];
        let expected = [
            SigElement::new(0.0, 1.0, 1.0),
            SigElement::new(1.0, 2.0, 0.0),
            SigElement::new(2.0, 3.0, 0.0),
            SigElement::new(3.0, 4.0, 1.0),
            SigElement::new(4.0, 5.0, -1.0),
            SigElement::new(5.0, 6.0, 0.0),
            SigElement::new(6.0, 7.0, 0.0),
            SigElement::new(7.0, 8.0, -1.0),
        ];

        let enc = Hdb3::build(1.0, -1.0).unwrap();
        assert_eq!(enc.encode(&seq).as_ref(), &expected);
    }

    #[test]
    fn test_hdb3_00010000() {
        let seq = [0, 0, 0, 1, 0, 0, 0, 0];
        let expected = [
            SigElement::new(0.0, 1.0, 0.0),
            SigElement::new(1.0, 2.0, 0.0),
            SigElement::new(2.0, 3.0, 0.0),
            SigElement::new(3.0, 4.0, 1.0),
            SigElement::new(4.0, 5.0, 0.0),
            SigElement::new(5.0, 6.0, 0.0),
            SigElement::new(6.0, 7.0, 0.0),
            SigElement::new(7.0, 8.0, 1.0),
        ];

        let enc = Hdb3::build(1.0, -1.0).unwrap();
        assert_eq!(enc.encode(&seq).as_ref(), &expected);
    }

    #[test]
    fn test_hdb3_mixed_parity_blocks() {
        let seq = [1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0];
        let expected = [
            SigElement::new(0.0, 1.0, 1.0),
            SigElement::new(1.0, 2.0, 0.0),
            SigElement::new(2.0, 3.0, 0.0),
            SigElement::new(3.0, 4.0, 0.0),
            SigElement::new(4.0, 5.0, 1.0),
            SigElement::new(5.0, 6.0, -1.0),
            SigElement::new(6.0, 7.0, 0.0),
            SigElement::new(7.0, 8.0, 0.0),
            SigElement::new(8.0, 9.0, -1.0),
            SigElement::new(9.0, 10.0, 1.0),
            SigElement::new(10.0, 11.0, 0.0),
            SigElement::new(11.0, 12.0, 0.0),
            SigElement::new(12.0, 13.0, 0.0),
            SigElement::new(13.0, 14.0, 1.0),
        ];

        let enc = Hdb3::build(1.0, -1.0).unwrap();
        assert_eq!(enc.encode(&seq).as_ref(), &expected);
    }

    #[test]
    fn test_hdb3_unarios_len1() {
        let enc = Hdb3::build(1.0, -1.0).unwrap();

        let s0 = [0u8; 1];
        let e0 = [SigElement::new(0.0, 1.0, 0.0)];
        assert_eq!(enc.encode(&s0).as_ref(), &e0);

        let s1 = [1u8; 1];
        let e1 = [SigElement::new(0.0, 1.0, 1.0)];
        assert_eq!(enc.encode(&s1).as_ref(), &e1);
    }
}
