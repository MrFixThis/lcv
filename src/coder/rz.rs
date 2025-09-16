use crate::util;

use super::{LineCoder, SignalElem};

#[derive(Debug, Default, Clone)]
pub struct Rz {
    tb: f32,
    v: f32,
    duty: f32,
}

impl Rz {
    pub fn build(tb: f32, v: f32, duty: f32) -> anyhow::Result<Self> {
        Ok(Self {
            tb: util::check_bit_period(tb)?,
            v: util::check_ampl_closed(v)?,
            duty: util::check_duty(duty)?,
        })
    }
}

impl LineCoder for Rz {
    fn encode(&self, bits: &[u8]) -> Box<[SignalElem]> {
        let h = self.tb * self.duty;
        let mut t = 0.0;
        let mut out = Vec::new();

        for &bit in bits {
            let tf0 = t + h;
            if tf0 > t {
                out.push(SignalElem::new(
                    t,
                    tf0,
                    if bit == 1 { self.v } else { -self.v },
                ));
            }

            let tf1 = t + self.tb;
            if tf1 > tf0 {
                out.push(SignalElem::new(tf0, tf1, 0.0));
            }

            t += self.tb;
        }

        out.into_boxed_slice()
    }

    fn on_tb(&mut self, tb: f32) -> anyhow::Result<()> {
        self.tb = util::check_bit_period(tb)?;
        Ok(())
    }

    fn on_v(&mut self, v: f32) -> anyhow::Result<()> {
        self.v = util::check_ampl_closed(v)?;
        Ok(())
    }

    fn on_duty(&mut self, duty: f32) -> anyhow::Result<()> {
        self.duty = util::check_duty(duty)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::coder::{LineCoder, SignalElem, rz::Rz};

    test_len_case!(test_rz_len4_cases: Rz::build(1.0, 1.0, 0.5).unwrap() => [
        ([0,0,0,0], [
            SignalElem::new(0.0,0.5,-1.0),
            SignalElem::new(0.5,1.0,0.0),
            SignalElem::new(1.0,1.5,-1.0),
            SignalElem::new(1.5,2.0,0.0),
            SignalElem::new(2.0,2.5,-1.0),
            SignalElem::new(2.5,3.0,0.0),
            SignalElem::new(3.0,3.5,-1.0),
            SignalElem::new(3.5,4.0,0.0),
        ]),
        ([1,1,1,1], [
            SignalElem::new(0.0,0.5,1.0),
            SignalElem::new(0.5,1.0,0.0),
            SignalElem::new(1.0,1.5,1.0),
            SignalElem::new(1.5,2.0,0.0),
            SignalElem::new(2.0,2.5,1.0),
            SignalElem::new(2.5,3.0,0.0),
            SignalElem::new(3.0,3.5,1.0),
            SignalElem::new(3.5,4.0,0.0),
        ]),
        ([1,0,1,0], [
            SignalElem::new(0.0,0.5,1.0),
            SignalElem::new(0.5,1.0,0.0),
            SignalElem::new(1.0,1.5,-1.0),
            SignalElem::new(1.5,2.0,0.0),
            SignalElem::new(2.0,2.5,1.0),
            SignalElem::new(2.5,3.0,0.0),
            SignalElem::new(3.0,3.5,-1.0),
            SignalElem::new(3.5,4.0,0.0),
        ]),
    ]);

    test_len_case!(test_rz_len6_cases: Rz::build(1.0, 1.0, 0.5).unwrap() => [
        ([0,1,1,1,1,0], [
            SignalElem::new(0.0,0.5,-1.0),
            SignalElem::new(0.5,1.0,0.0),
            SignalElem::new(1.0,1.5,1.0),
            SignalElem::new(1.5,2.0,0.0),
            SignalElem::new(2.0,2.5,1.0),
            SignalElem::new(2.5,3.0,0.0),
            SignalElem::new(3.0,3.5,1.0),
            SignalElem::new(3.5,4.0,0.0),
            SignalElem::new(4.0,4.5,1.0),
            SignalElem::new(4.5,5.0,0.0),
            SignalElem::new(5.0,5.5,-1.0),
            SignalElem::new(5.5,6.0,0.0),
        ]),
        ([1,0,1,0,1,0], [
            SignalElem::new(0.0,0.5,1.0),
            SignalElem::new(0.5,1.0,0.0),
            SignalElem::new(1.0,1.5,-1.0),
            SignalElem::new(1.5,2.0,0.0),
            SignalElem::new(2.0,2.5,1.0),
            SignalElem::new(2.5,3.0,0.0),
            SignalElem::new(3.0,3.5,-1.0),
            SignalElem::new(3.5,4.0,0.0),
            SignalElem::new(4.0,4.5,1.0),
            SignalElem::new(4.5,5.0,0.0),
            SignalElem::new(5.0,5.5,-1.0),
            SignalElem::new(5.5,6.0,0.0),
        ]),
    ]);

    #[test]
    fn test_rz_unaries_len1() {
        let enc = Rz::build(1.0, 1.0, 0.5).unwrap();

        let s0 = [0u8; 1];
        let e0 = [SignalElem::new(0.0, 0.5, -1.0), SignalElem::new(0.5, 1.0, 0.0)];
        assert_eq!(enc.encode(&s0).as_ref(), &e0);

        let s1 = [1u8; 1];
        let e1 = [SignalElem::new(0.0, 0.5, 1.0), SignalElem::new(0.5, 1.0, 0.0)];
        assert_eq!(enc.encode(&s1).as_ref(), &e1);
    }
}
