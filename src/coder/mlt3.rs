use crate::util;

use super::{LineCoder, SigElement};

#[derive(Debug, Clone, Copy)]
pub struct Mlt3 {
    tb: f64,
    v: f64,
}

impl Default for Mlt3 {
    fn default() -> Self {
        Self { tb: 1.0, v: 1.0 }
    }
}

impl Mlt3 {
    pub fn build(tb: f64, v: f64) -> anyhow::Result<Self> {
        Ok(Self {
            tb: util::check_bit_period(tb)?,
            v: util::check_ampl_closed(v)?,
        })
    }
}

impl LineCoder for Mlt3 {
    fn encode(&self, bits: &[u8]) -> Box<[SigElement]> {
        let cycle = [0.0, self.v, 0.0, -self.v];
        let mut idx = 0;
        let mut t = 0.0;

        bits.iter()
            .filter_map(|&bit| {
                if bit == 1 {
                    idx = (idx + 1) & 3;
                }

                let mut res = None;
                let tf = t + self.tb;
                if tf > t {
                    res = Some(SigElement::new(t, tf, cycle[idx]));
                }

                t += self.tb;
                res
            })
            .collect()
    }

    fn on_tb(&mut self, tb: f64) -> anyhow::Result<()> {
        self.tb = util::check_bit_period(tb)?;
        Ok(())
    }

    fn on_v(&mut self, v: f64) -> anyhow::Result<()> {
        self.v = util::check_ampl_closed(v)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::coder::{LineCoder, SigElement, mlt3::Mlt3};

    crate::test_len_case!(test_mlt3_len4_cases: Mlt3::build(1.0, 1.0).unwrap() => [
        ([0,0,0,0], [
            SigElement::new(0.0,1.0, 0.0),
            SigElement::new(1.0,2.0, 0.0),
            SigElement::new(2.0,3.0, 0.0),
            SigElement::new(3.0,4.0, 0.0),
        ]),
        ([1,1,1,1], [
            SigElement::new(0.0,1.0, 1.0),
            SigElement::new(1.0,2.0, 0.0),
            SigElement::new(2.0,3.0,-1.0),
            SigElement::new(3.0,4.0, 0.0),
        ]),
        ([1,0,1,0], [
            SigElement::new(0.0,1.0, 1.0),
            SigElement::new(1.0,2.0, 1.0),
            SigElement::new(2.0,3.0, 0.0),
            SigElement::new(3.0,4.0, 0.0),
        ]),
        ([0,1,1,0], [
            SigElement::new(0.0,1.0, 0.0),
            SigElement::new(1.0,2.0, 1.0),
            SigElement::new(2.0,3.0, 0.0),
            SigElement::new(3.0,4.0, 0.0),
        ]),
    ]);

    crate::test_len_case!(test_mlt3_len6_cases: Mlt3::build(1.0, 1.0).unwrap() => [
        ([1,0,1,0,1,0], [
            SigElement::new(0.0,1.0, 1.0),
            SigElement::new(1.0,2.0, 1.0),
            SigElement::new(2.0,3.0, 0.0),
            SigElement::new(3.0,4.0, 0.0),
            SigElement::new(4.0,5.0,-1.0),
            SigElement::new(5.0,6.0,-1.0),
        ]),
        ([0,1,1,1,1,0], [
            SigElement::new(0.0,1.0, 0.0),
            SigElement::new(1.0,2.0, 1.0),
            SigElement::new(2.0,3.0, 0.0),
            SigElement::new(3.0,4.0,-1.0),
            SigElement::new(4.0,5.0, 0.0),
            SigElement::new(5.0,6.0, 0.0),
        ]),
    ]);

    crate::test_len_case!(test_mlt3_len8_cases: Mlt3::build(1.0, 1.0).unwrap() => [
        ([0,0,1,1,0,0,1,1], [
            SigElement::new(0.0,1.0, 0.0),
            SigElement::new(1.0,2.0, 0.0),
            SigElement::new(2.0,3.0, 1.0),
            SigElement::new(3.0,4.0, 0.0),
            SigElement::new(4.0,5.0, 0.0),
            SigElement::new(5.0,6.0, 0.0),
            SigElement::new(6.0,7.0,-1.0),
            SigElement::new(7.0,8.0, 0.0),
        ]),
        ([0,1,0,1,0,1,1,1], [
            SigElement::new(0.0,1.0, 0.0),
            SigElement::new(1.0,2.0, 1.0),
            SigElement::new(2.0,3.0, 1.0),
            SigElement::new(3.0,4.0, 0.0),
            SigElement::new(4.0,5.0, 0.0),
            SigElement::new(5.0,6.0,-1.0),
            SigElement::new(6.0,7.0, 0.0),
            SigElement::new(7.0,8.0, 1.0),
        ]),
    ]);

    #[test]
    fn test_mlt3_len9_edge() {
        let seq = [1, 1, 1, 0, 0, 0, 1, 1, 1];
        let exp = [
            SigElement::new(0.0, 1.0, 1.0),
            SigElement::new(1.0, 2.0, 0.0),
            SigElement::new(2.0, 3.0, -1.0),
            SigElement::new(3.0, 4.0, -1.0),
            SigElement::new(4.0, 5.0, -1.0),
            SigElement::new(5.0, 6.0, -1.0),
            SigElement::new(6.0, 7.0, 0.0),
            SigElement::new(7.0, 8.0, 1.0),
            SigElement::new(8.0, 9.0, 0.0),
        ];

        let enc = Mlt3::build(1.0, 1.0).unwrap();
        assert_eq!(enc.encode(&seq).as_ref(), &exp);
    }

    #[test]
    fn test_mlt3_unarios_len1() {
        let enc = Mlt3::build(1.0, 1.0).unwrap();

        let s0 = [0u8; 1];
        let e0 = [SigElement::new(0.0, 1.0, 0.0)];
        assert_eq!(enc.encode(&s0).as_ref(), &e0);

        let s1 = [1u8; 1];
        let e1 = [SigElement::new(0.0, 1.0, 1.0)];
        assert_eq!(enc.encode(&s1).as_ref(), &e1);
    }
}
