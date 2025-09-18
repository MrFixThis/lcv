use crate::util;

use super::{LineCoder, SignalElem};

#[derive(Debug, Clone, Copy)]
pub struct Ami {
    tb: f32,
    v: f32,
}

impl Default for Ami {
    fn default() -> Self {
        Self { tb: 1.0, v: 1.0 }
    }
}

impl Ami {
    pub fn build(tb: f32, v: f32) -> anyhow::Result<Self> {
        Ok(Self {
            tb: util::check_bit_period(tb)?,
            v: util::check_ampl_opened(v)?,
        })
    }
}

impl LineCoder for Ami {
    fn encode(&self, bits: &[u8]) -> Box<[SignalElem]> {
        let mut t = 0.0;
        let mut lvl = self.v;

        bits.iter()
            .filter_map(|&bit| {
                let mut res = None;
                let tf = t + self.tb;
                if bit == 1 {
                    lvl = -lvl;
                    if tf > t {
                        res = Some(SignalElem::new(t, tf, lvl));
                    }
                } else if tf > t {
                    res = Some(SignalElem::new(t, tf, 0.0));
                }

                t += self.tb;
                res
            })
            .collect()
    }

    fn on_tb(&mut self, tb: f32) -> anyhow::Result<()> {
        self.tb = util::check_bit_period(tb)?;
        Ok(())
    }

    fn on_v(&mut self, v: f32) -> anyhow::Result<()> {
        self.v = util::check_ampl_opened(v)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::coder::{LineCoder, SignalElem, ami::Ami};

    crate::test_len_case!(test_ami_len4_cases: Ami::build(1.0, -1.0).unwrap() => [
        ([0,0,0,0], [
            SignalElem::new(0.0,1.0, 0.0),
            SignalElem::new(1.0,2.0, 0.0),
            SignalElem::new(2.0,3.0, 0.0),
            SignalElem::new(3.0,4.0, 0.0),
        ]),
        ([1,1,1,1], [
            SignalElem::new(0.0,1.0, 1.0),
            SignalElem::new(1.0,2.0,-1.0),
            SignalElem::new(2.0,3.0, 1.0),
            SignalElem::new(3.0,4.0,-1.0),
        ]),
        ([1,0,1,0], [
            SignalElem::new(0.0,1.0, 1.0),
            SignalElem::new(1.0,2.0, 0.0),
            SignalElem::new(2.0,3.0,-1.0),
            SignalElem::new(3.0,4.0, 0.0),
        ]),
    ]);

    crate::test_len_case!(test_ami_len6_cases: Ami::build(1.0, -1.0).unwrap() => [
        ([0,1,1,1,1,0], [
            SignalElem::new(0.0,1.0, 0.0),
            SignalElem::new(1.0,2.0, 1.0),
            SignalElem::new(2.0,3.0,-1.0),
            SignalElem::new(3.0,4.0, 1.0),
            SignalElem::new(4.0,5.0,-1.0),
            SignalElem::new(5.0,6.0, 0.0),
        ]),
        ([1,0,1,0,1,0], [
            SignalElem::new(0.0,1.0, 1.0),
            SignalElem::new(1.0,2.0, 0.0),
            SignalElem::new(2.0,3.0,-1.0),
            SignalElem::new(3.0,4.0, 0.0),
            SignalElem::new(4.0,5.0, 1.0),
            SignalElem::new(5.0,6.0, 0.0),
        ]),
    ]);

    crate::test_len_case!(test_ami_len8_cases: Ami::build(1.0, -1.0).unwrap() => [
        ([0,0,1,1,0,0,1,1], [
            SignalElem::new(0.0,1.0, 0.0),
            SignalElem::new(1.0,2.0, 0.0),
            SignalElem::new(2.0,3.0, 1.0),
            SignalElem::new(3.0,4.0,-1.0),
            SignalElem::new(4.0,5.0, 0.0),
            SignalElem::new(5.0,6.0, 0.0),
            SignalElem::new(6.0,7.0, 1.0),
            SignalElem::new(7.0,8.0,-1.0),
        ]),
        ([0,1,0,1,0,1,1,1], [
            SignalElem::new(0.0,1.0, 0.0),
            SignalElem::new(1.0,2.0, 1.0),
            SignalElem::new(2.0,3.0, 0.0),
            SignalElem::new(3.0,4.0,-1.0),
            SignalElem::new(4.0,5.0, 0.0),
            SignalElem::new(5.0,6.0, 1.0),
            SignalElem::new(6.0,7.0,-1.0),
            SignalElem::new(7.0,8.0, 1.0),
        ]),
    ]);

    #[test]
    fn test_ami_runs_of_zeroes() {
        let seq = [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1];
        let exp = [
            SignalElem::new(0.0, 1.0, 0.0),
            SignalElem::new(1.0, 2.0, 0.0),
            SignalElem::new(2.0, 3.0, 0.0),
            SignalElem::new(3.0, 4.0, 1.0),
            SignalElem::new(4.0, 5.0, 0.0),
            SignalElem::new(5.0, 6.0, 0.0),
            SignalElem::new(6.0, 7.0, 0.0),
            SignalElem::new(7.0, 8.0, -1.0),
            SignalElem::new(8.0, 9.0, 0.0),
            SignalElem::new(9.0, 10.0, 0.0),
            SignalElem::new(10.0, 11.0, 0.0),
            SignalElem::new(11.0, 12.0, 1.0),
        ];

        let enc = Ami::build(1.0, -1.0).unwrap();
        assert_eq!(enc.encode(&seq).as_ref(), &exp);
    }

    #[test]
    fn test_ami_unarios_len1() {
        let enc = Ami::build(1.0, -1.0).unwrap();

        let s0 = [0u8; 1];
        let e0 = [SignalElem::new(0.0, 1.0, 0.0)];
        assert_eq!(enc.encode(&s0).as_ref(), &e0);

        let s1 = [1u8; 1];
        let e1 = [SignalElem::new(0.0, 1.0, 1.0)];
        assert_eq!(enc.encode(&s1).as_ref(), &e1);
    }
}
