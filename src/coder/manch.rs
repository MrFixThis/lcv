use crate::{coder::SignalElem, util};

use super::LineCoder;

#[derive(Debug, Clone, Copy)]
pub struct Manchester {
    tb: f32,
    v: f32,
}

impl Default for Manchester {
    fn default() -> Self {
        Self { tb: 1.0, v: 1.0 }
    }
}

impl Manchester {
    const DEF_DUTY: f32 = 0.5;

    pub fn build(tb: f32, v: f32) -> anyhow::Result<Self> {
        Ok(Self {
            tb: util::check_bit_period(tb)?,
            v: util::check_ampl_closed(v)?,
        })
    }
}

impl LineCoder for Manchester {
    fn encode(&self, bits: &[u8]) -> Box<[SignalElem]> {
        let h = self.tb * Self::DEF_DUTY;
        let mut t = 0.0;
        let mut out = Vec::new();

        for &bit in bits {
            if bit == 0 {
                let t0 = t + h;
                if t0 > t {
                    out.push(SignalElem::new(t, t0, self.v));
                }

                let t1 = t + self.tb;
                if t1 > t0 {
                    out.push(SignalElem::new(t0, t1, -self.v));
                }
            } else {
                let t0 = t + h;
                if t0 > t {
                    out.push(SignalElem::new(t, t0, -self.v));
                }

                let t1 = t + self.tb;
                if t1 > t0 {
                    out.push(SignalElem::new(t0, t1, self.v));
                }
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
}

#[cfg(test)]
mod tests {
    use crate::coder::{LineCoder, SignalElem, manch::Manchester};

    test_len_case!(test_manchester_len4_cases: Manchester::build(1.0, 1.0).unwrap() => [
        ([0,0,0,0], [
            SignalElem::new(0.0,0.5, 1.0),
            SignalElem::new(0.5,1.0,-1.0),
            SignalElem::new(1.0,1.5, 1.0),
            SignalElem::new(1.5,2.0,-1.0),
            SignalElem::new(2.0,2.5, 1.0),
            SignalElem::new(2.5,3.0,-1.0),
            SignalElem::new(3.0,3.5, 1.0),
            SignalElem::new(3.5,4.0,-1.0),
        ]),
        ([1,1,1,1], [
            SignalElem::new(0.0,0.5,-1.0),
            SignalElem::new(0.5,1.0, 1.0),
            SignalElem::new(1.0,1.5,-1.0),
            SignalElem::new(1.5,2.0, 1.0),
            SignalElem::new(2.0,2.5,-1.0),
            SignalElem::new(2.5,3.0, 1.0),
            SignalElem::new(3.0,3.5,-1.0),
            SignalElem::new(3.5,4.0, 1.0),
        ]),
        ([1,0,1,0], [
            SignalElem::new(0.0,0.5,-1.0),
            SignalElem::new(0.5,1.0, 1.0),
            SignalElem::new(1.0,1.5, 1.0),
            SignalElem::new(1.5,2.0,-1.0),
            SignalElem::new(2.0,2.5,-1.0),
            SignalElem::new(2.5,3.0, 1.0),
            SignalElem::new(3.0,3.5, 1.0),
            SignalElem::new(3.5,4.0,-1.0),
        ]),
    ]);

    test_len_case!(test_manchester_len6_cases: Manchester::build(1.0, 1.0).unwrap() => [
        ([0,1,1,1,1,0], [
            SignalElem::new(0.0,0.5, 1.0),
            SignalElem::new(0.5,1.0,-1.0),
            SignalElem::new(1.0,1.5,-1.0),
            SignalElem::new(1.5,2.0, 1.0),
            SignalElem::new(2.0,2.5,-1.0),
            SignalElem::new(2.5,3.0, 1.0),
            SignalElem::new(3.0,3.5,-1.0),
            SignalElem::new(3.5,4.0, 1.0),
            SignalElem::new(4.0,4.5,-1.0),
            SignalElem::new(4.5,5.0, 1.0),
            SignalElem::new(5.0,5.5, 1.0),
            SignalElem::new(5.5,6.0,-1.0),
        ]),
        ([1,0,1,0,1,0], [
            SignalElem::new(0.0,0.5,-1.0),
            SignalElem::new(0.5,1.0, 1.0),
            SignalElem::new(1.0,1.5, 1.0),
            SignalElem::new(1.5,2.0,-1.0),
            SignalElem::new(2.0,2.5,-1.0),
            SignalElem::new(2.5,3.0, 1.0),
            SignalElem::new(3.0,3.5, 1.0),
            SignalElem::new(3.5,4.0,-1.0),
            SignalElem::new(4.0,4.5,-1.0),
            SignalElem::new(4.5,5.0, 1.0),
            SignalElem::new(5.0,5.5, 1.0),
            SignalElem::new(5.5,6.0,-1.0),
        ]),
    ]);

    test_len_case!(test_manchester_len8_cases: Manchester::build(1.0, 1.0).unwrap() => [
        ([0,0,1,1,0,0,1,1], [
            SignalElem::new(0.0,0.5, 1.0),
            SignalElem::new(0.5,1.0,-1.0),
            SignalElem::new(1.0,1.5, 1.0),
            SignalElem::new(1.5,2.0,-1.0),
            SignalElem::new(2.0,2.5,-1.0),
            SignalElem::new(2.5,3.0, 1.0),
            SignalElem::new(3.0,3.5,-1.0),
            SignalElem::new(3.5,4.0, 1.0),
            SignalElem::new(4.0,4.5, 1.0),
            SignalElem::new(4.5,5.0,-1.0),
            SignalElem::new(5.0,5.5, 1.0),
            SignalElem::new(5.5,6.0,-1.0),
            SignalElem::new(6.0,6.5,-1.0),
            SignalElem::new(6.5,7.0, 1.0),
            SignalElem::new(7.0,7.5,-1.0),
            SignalElem::new(7.5,8.0, 1.0),
        ]),
        ([0,1,0,1,0,1,1,1], [
            SignalElem::new(0.0,0.5, 1.0),
            SignalElem::new(0.5,1.0,-1.0),
            SignalElem::new(1.0,1.5,-1.0),
            SignalElem::new(1.5,2.0, 1.0),
            SignalElem::new(2.0,2.5, 1.0),
            SignalElem::new(2.5,3.0,-1.0),
            SignalElem::new(3.0,3.5,-1.0),
            SignalElem::new(3.5,4.0, 1.0),
            SignalElem::new(4.0,4.5, 1.0),
            SignalElem::new(4.5,5.0,-1.0),
            SignalElem::new(5.0,5.5,-1.0),
            SignalElem::new(5.5,6.0, 1.0),
            SignalElem::new(6.0,6.5,-1.0),
            SignalElem::new(6.5,7.0, 1.0),
            SignalElem::new(7.0,7.5,-1.0),
            SignalElem::new(7.5,8.0, 1.0),
        ]),
    ]);

    #[test]
    fn test_manchester_unarios_len1() {
        let enc = Manchester::build(1.0, 1.0).unwrap();

        let s0 = [0u8; 1];
        let e0 = [
            SignalElem::new(0.0, 0.5, 1.0),
            SignalElem::new(0.5, 1.0, -1.0),
        ];
        assert_eq!(enc.encode(&s0).as_ref(), &e0);

        let s1 = [1u8; 1];
        let e1 = [
            SignalElem::new(0.0, 0.5, -1.0),
            SignalElem::new(0.5, 1.0, 1.0),
        ];
        assert_eq!(enc.encode(&s1).as_ref(), &e1);
    }
}
