use crate::util;

use super::{LineCoder, SignalElem};

#[derive(Debug, Clone, Copy)]
pub struct Nrzl {
    tb: f32,
    v: f32,
}

impl Default for Nrzl {
    fn default() -> Self {
        Self { tb: 1.0, v: 1.0 }
    }
}

impl Nrzl {
    pub fn build(tb: f32, v: f32) -> anyhow::Result<Self> {
        Ok(Self {
            tb: util::check_bit_period(tb)?,
            v: util::check_ampl_closed(v)?,
        })
    }

    pub fn set_tb(&mut self, tb: f32) -> bool {
        let Ok(tb) = util::check_bit_period(tb) else {
            return false;
        };
        self.tb = tb;
        true
    }
}

impl LineCoder for Nrzl {
    fn encode(&self, bits: &[u8]) -> Box<[SignalElem]> {
        let mut t = 0.0;
        bits.iter()
            .filter_map(|&bit| {
                let mut res = None;
                let tf = t + self.tb;
                if tf > t {
                    let lvl = if bit == 1 { self.v } else { -self.v };
                    res = Some(SignalElem::new(t, tf, lvl));
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
        self.v = util::check_ampl_closed(v)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Nrzi {
    tb: f32,
    v: f32,
}

impl Default for Nrzi {
    fn default() -> Self {
        Self { tb: 1.0, v: 1.0 }
    }
}

impl Nrzi {
    pub fn build(tb: f32, v: f32) -> anyhow::Result<Self> {
        Ok(Self {
            tb: util::check_bit_period(tb)?,
            v: util::check_ampl_opened(v)?,
        })
    }
}

impl LineCoder for Nrzi {
    fn encode(&self, bits: &[u8]) -> Box<[SignalElem]> {
        let mut t = 0.0;
        let mut lvl = self.v;
        bits.iter()
            .filter_map(|&bit| {
                let mut res = None;
                let tf = t + self.tb;
                if tf > t {
                    if bit == 1 {
                        lvl = -lvl;
                    }
                    res = Some(SignalElem::new(t, tf, lvl));
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
    use super::{Nrzi, Nrzl};
    use crate::coder::{LineCoder, SignalElem};

    test_len_case!(test_nrzl_len4_cases: Nrzl::build(1.0, 1.0).unwrap() => [
        ([0, 0, 0, 0],
            [
                SignalElem::new(0.0, 1.0, -1.0),
                SignalElem::new(1.0, 2.0, -1.0),
                SignalElem::new(2.0, 3.0, -1.0),
                SignalElem::new(3.0, 4.0, -1.0),
            ]
        ),
        ([1, 1, 1, 1],
            [
                SignalElem::new(0.0, 1.0, 1.0),
                SignalElem::new(1.0, 2.0, 1.0),
                SignalElem::new(2.0, 3.0, 1.0),
                SignalElem::new(3.0, 4.0, 1.0),
            ]
        ),
        ([1, 0, 1, 0],
            [
                SignalElem::new(0.0, 1.0, 1.0),
                SignalElem::new(1.0, 2.0, -1.0),
                SignalElem::new(2.0, 3.0, 1.0),
                SignalElem::new(3.0, 4.0, -1.0),
            ]
        ),
        ([0, 1, 1, 0],
            [
                SignalElem::new(0.0, 1.0, -1.0),
                SignalElem::new(1.0, 2.0, 1.0),
                SignalElem::new(2.0, 3.0, 1.0),
                SignalElem::new(3.0, 4.0, -1.0),
            ],
        ),
    ]);

    test_len_case!(test_nrzi_len4_cases: Nrzi::build(1.0, -1.0).unwrap() => [
        ([0, 0, 0, 0],
            [
                SignalElem::new(0.0, 1.0, -1.0),
                SignalElem::new(1.0, 2.0, -1.0),
                SignalElem::new(2.0, 3.0, -1.0),
                SignalElem::new(3.0, 4.0, -1.0),
            ]
        ),
        ([1, 1, 1, 1],
            [
                SignalElem::new(0.0, 1.0, 1.0),
                SignalElem::new(1.0, 2.0, -1.0),
                SignalElem::new(2.0, 3.0, 1.0),
                SignalElem::new(3.0, 4.0, -1.0),
            ]
        ),
        ([1, 0, 1, 0],
            [
                SignalElem::new(0.0, 1.0, 1.0),
                SignalElem::new(1.0, 2.0, 1.0),
                SignalElem::new(2.0, 3.0, -1.0),
                SignalElem::new(3.0, 4.0, -1.0),
            ]
        ),
        ([0, 1, 1, 0],
            [
                SignalElem::new(0.0, 1.0, -1.0),
                SignalElem::new(1.0, 2.0, 1.0),
                SignalElem::new(2.0, 3.0, -1.0),
                SignalElem::new(3.0, 4.0, -1.0),
            ],
        ),
    ]);

    test_len_case!(test_nrzl_len6_cases: Nrzl::build(1.0, 1.0).unwrap() => [
        ([1, 0, 1, 0, 1, 0],
            [
                SignalElem::new(0.0, 1.0, 1.0),
                SignalElem::new(1.0, 2.0, -1.0),
                SignalElem::new(2.0, 3.0, 1.0),
                SignalElem::new(3.0, 4.0, -1.0),
                SignalElem::new(4.0, 5.0, 1.0),
                SignalElem::new(5.0, 6.0, -1.0),
            ]
        ),
        ([0, 1, 1, 1, 1, 0],
            [
                SignalElem::new(0.0, 1.0, -1.0),
                SignalElem::new(1.0, 2.0, 1.0),
                SignalElem::new(2.0, 3.0, 1.0),
                SignalElem::new(3.0, 4.0, 1.0),
                SignalElem::new(4.0, 5.0, 1.0),
                SignalElem::new(5.0, 6.0, -1.0),
            ]
        ),
    ]);

    test_len_case!(test_nrzi_len6_cases: Nrzi::build(1.0, -1.0).unwrap() => [
        ([1, 0, 1, 0, 1, 0],
            [
                SignalElem::new(0.0, 1.0, 1.0),
                SignalElem::new(1.0, 2.0, 1.0),
                SignalElem::new(2.0, 3.0, -1.0),
                SignalElem::new(3.0, 4.0, -1.0),
                SignalElem::new(4.0, 5.0, 1.0),
                SignalElem::new(5.0, 6.0, 1.0),
            ]
        ),
        ([0, 1, 1, 1, 1, 0],
            [
                SignalElem::new(0.0, 1.0, -1.0),
                SignalElem::new(1.0, 2.0, 1.0),
                SignalElem::new(2.0, 3.0, -1.0),
                SignalElem::new(3.0, 4.0, 1.0),
                SignalElem::new(4.0, 5.0, -1.0),
                SignalElem::new(5.0, 6.0, -1.0),
            ]
        ),
    ]);

    test_len_case!(test_nrzl_len8_cases: Nrzl::build(1.0, 1.0).unwrap() => [
        ([0, 0, 1, 1, 0, 0, 1, 1],
            [
                SignalElem::new(0.0, 1.0, -1.0),
                SignalElem::new(1.0, 2.0, -1.0),
                SignalElem::new(2.0, 3.0, 1.0),
                SignalElem::new(3.0, 4.0, 1.0),
                SignalElem::new(4.0, 5.0, -1.0),
                SignalElem::new(5.0, 6.0, -1.0),
                SignalElem::new(6.0, 7.0, 1.0),
                SignalElem::new(7.0, 8.0, 1.0),
            ]
        ),
        ([0, 1, 0, 1, 0, 1, 1, 1],
            [
                SignalElem::new(0.0, 1.0, -1.0),
                SignalElem::new(1.0, 2.0, 1.0),
                SignalElem::new(2.0, 3.0, -1.0),
                SignalElem::new(3.0, 4.0, 1.0),
                SignalElem::new(4.0, 5.0, -1.0),
                SignalElem::new(5.0, 6.0, 1.0),
                SignalElem::new(6.0, 7.0, 1.0),
                SignalElem::new(7.0, 8.0, 1.0),
            ]
        ),
    ]);

    test_len_case!(test_nrzi_len8_cases: Nrzi::build(1.0, -1.0).unwrap() => [
        ([0, 0, 1, 1, 0, 0, 1, 1],
            [
                SignalElem::new(0.0, 1.0, -1.0),
                SignalElem::new(1.0, 2.0, -1.0),
                SignalElem::new(2.0, 3.0, 1.0),
                SignalElem::new(3.0, 4.0, -1.0),
                SignalElem::new(4.0, 5.0, -1.0),
                SignalElem::new(5.0, 6.0, -1.0),
                SignalElem::new(6.0, 7.0, 1.0),
                SignalElem::new(7.0, 8.0, -1.0),
            ]
        ),
        ([0, 1, 0, 1, 0, 1, 1, 1],
            [
                SignalElem::new(0.0, 1.0, -1.0),
                SignalElem::new(1.0, 2.0, 1.0),
                SignalElem::new(2.0, 3.0, 1.0),
                SignalElem::new(3.0, 4.0, -1.0),
                SignalElem::new(4.0, 5.0, -1.0),
                SignalElem::new(5.0, 6.0, 1.0),
                SignalElem::new(6.0, 7.0, -1.0),
                SignalElem::new(7.0, 8.0, 1.0),
            ]
        ),
    ]);

    #[test]
    fn test_len9_edge_patterns() {
        let seq = [1, 1, 1, 0, 0, 0, 1, 1, 1];
        let exp_l = [
            SignalElem::new(0.0, 1.0, 1.0),
            SignalElem::new(1.0, 2.0, 1.0),
            SignalElem::new(2.0, 3.0, 1.0),
            SignalElem::new(3.0, 4.0, -1.0),
            SignalElem::new(4.0, 5.0, -1.0),
            SignalElem::new(5.0, 6.0, -1.0),
            SignalElem::new(6.0, 7.0, 1.0),
            SignalElem::new(7.0, 8.0, 1.0),
            SignalElem::new(8.0, 9.0, 1.0),
        ];
        let exp_i = [
            SignalElem::new(0.0, 1.0, 1.0),
            SignalElem::new(1.0, 2.0, -1.0),
            SignalElem::new(2.0, 3.0, 1.0),
            SignalElem::new(3.0, 4.0, 1.0),
            SignalElem::new(4.0, 5.0, 1.0),
            SignalElem::new(5.0, 6.0, 1.0),
            SignalElem::new(6.0, 7.0, -1.0),
            SignalElem::new(7.0, 8.0, 1.0),
            SignalElem::new(8.0, 9.0, -1.0),
        ];

        let nrzl = Nrzl::build(1.0, 1.0).unwrap();
        let nrzi = Nrzi::build(1.0, -1.0).unwrap();

        assert_eq!(nrzl.encode(&seq).as_ref(), &exp_l);
        assert_eq!(nrzi.encode(&seq).as_ref(), &exp_i);
    }

    #[test]
    fn test_unarios_len1() {
        let nrzl = Nrzl::build(1.0, 1.0).unwrap();
        let nrzi = Nrzi::build(1.0, -1.0).unwrap();

        let s0 = [0u8; 1];
        let e0 = [SignalElem::new(0.0, 1.0, -1.0)];
        assert_eq!(nrzl.encode(&s0).as_ref(), &e0);
        assert_eq!(nrzi.encode(&s0).as_ref(), &e0);

        let s1 = [1u8; 1];
        let e1_l = [SignalElem::new(0.0, 1.0, 1.0)];
        let e1_i = [SignalElem::new(0.0, 1.0, 1.0)];
        assert_eq!(nrzl.encode(&s1).as_ref(), &e1_l);
        assert_eq!(nrzi.encode(&s1).as_ref(), &e1_i);
    }
}
