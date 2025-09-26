use super::{LineCoder, SigElement};

#[derive(Debug, Clone, Copy)]
pub struct Nrzl {
    tb: f64,
    v: f64,
}

impl Default for Nrzl {
    fn default() -> Self {
        Self {
            tb: super::GLOB_BASE_TB,
            v: super::GLOB_BASE_V,
        }
    }
}

impl Nrzl {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

impl LineCoder for Nrzl {
    fn encode(&self, bits: &[u8]) -> Box<[SigElement]> {
        let mut t = 0.0;
        bits.iter()
            .filter_map(|&bit| {
                let mut res = None;
                let tf = t + self.tb;
                if tf > t {
                    let lvl = if bit == 1 { self.v } else { -self.v };
                    res = Some(SigElement::new(t, tf, lvl));
                }

                t += self.tb;
                res
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Nrzi {
    tb: f64,
    v: f64,
}

impl Default for Nrzi {
    fn default() -> Self {
        Self {
            tb: super::GLOB_BASE_TB,
            v: super::GLOB_BASE_V,
        }
    }
}

impl Nrzi {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

impl LineCoder for Nrzi {
    fn encode(&self, bits: &[u8]) -> Box<[SigElement]> {
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
                    res = Some(SigElement::new(t, tf, lvl));
                }

                t += self.tb;
                res
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Nrzi, Nrzl};
    use crate::coder::{LineCoder, SigElement};

    crate::test_len_case!(test_nrzl_len4_cases: Nrzl::new() => [
        ([0, 0, 0, 0],
            [
                SigElement::new(0.0, 1.0, -1.0),
                SigElement::new(1.0, 2.0, -1.0),
                SigElement::new(2.0, 3.0, -1.0),
                SigElement::new(3.0, 4.0, -1.0),
            ]
        ),
        ([1, 1, 1, 1],
            [
                SigElement::new(0.0, 1.0, 1.0),
                SigElement::new(1.0, 2.0, 1.0),
                SigElement::new(2.0, 3.0, 1.0),
                SigElement::new(3.0, 4.0, 1.0),
            ]
        ),
        ([1, 0, 1, 0],
            [
                SigElement::new(0.0, 1.0, 1.0),
                SigElement::new(1.0, 2.0, -1.0),
                SigElement::new(2.0, 3.0, 1.0),
                SigElement::new(3.0, 4.0, -1.0),
            ]
        ),
        ([0, 1, 1, 0],
            [
                SigElement::new(0.0, 1.0, -1.0),
                SigElement::new(1.0, 2.0, 1.0),
                SigElement::new(2.0, 3.0, 1.0),
                SigElement::new(3.0, 4.0, -1.0),
            ],
        ),
    ]);

    crate::test_len_case!(test_nrzi_len4_cases: Nrzi::new() => [
        ([0, 0, 0, 0],
            [
                SigElement::new(0.0, 1.0, 1.0),
                SigElement::new(1.0, 2.0, 1.0),
                SigElement::new(2.0, 3.0, 1.0),
                SigElement::new(3.0, 4.0, 1.0),
            ]
        ),
        ([1, 1, 1, 1],
            [
                SigElement::new(0.0, 1.0, -1.0),
                SigElement::new(1.0, 2.0, 1.0),
                SigElement::new(2.0, 3.0, -1.0),
                SigElement::new(3.0, 4.0, 1.0),
            ]
        ),
        ([1, 0, 1, 0],
            [
                SigElement::new(0.0, 1.0, -1.0),
                SigElement::new(1.0, 2.0, -1.0),
                SigElement::new(2.0, 3.0, 1.0),
                SigElement::new(3.0, 4.0, 1.0),
            ]
        ),
        ([0, 1, 1, 0],
            [
                SigElement::new(0.0, 1.0, 1.0),
                SigElement::new(1.0, 2.0, -1.0),
                SigElement::new(2.0, 3.0, 1.0),
                SigElement::new(3.0, 4.0, 1.0),
            ],
        ),
    ]);

    crate::test_len_case!(test_nrzl_len6_cases: Nrzl::new() => [
        ([1, 0, 1, 0, 1, 0],
            [
                SigElement::new(0.0, 1.0, 1.0),
                SigElement::new(1.0, 2.0, -1.0),
                SigElement::new(2.0, 3.0, 1.0),
                SigElement::new(3.0, 4.0, -1.0),
                SigElement::new(4.0, 5.0, 1.0),
                SigElement::new(5.0, 6.0, -1.0),
            ]
        ),
        ([0, 1, 1, 1, 1, 0],
            [
                SigElement::new(0.0, 1.0, -1.0),
                SigElement::new(1.0, 2.0, 1.0),
                SigElement::new(2.0, 3.0, 1.0),
                SigElement::new(3.0, 4.0, 1.0),
                SigElement::new(4.0, 5.0, 1.0),
                SigElement::new(5.0, 6.0, -1.0),
            ]
        ),
    ]);

    crate::test_len_case!(test_nrzi_len6_cases: Nrzi::new() => [
        ([1, 0, 1, 0, 1, 0],
            [
                SigElement::new(0.0, 1.0, -1.0),
                SigElement::new(1.0, 2.0, -1.0),
                SigElement::new(2.0, 3.0, 1.0),
                SigElement::new(3.0, 4.0, 1.0),
                SigElement::new(4.0, 5.0, -1.0),
                SigElement::new(5.0, 6.0, -1.0),
            ]
        ),
        ([0, 1, 1, 1, 1, 0],
            [
                SigElement::new(0.0, 1.0, 1.0),
                SigElement::new(1.0, 2.0, -1.0),
                SigElement::new(2.0, 3.0, 1.0),
                SigElement::new(3.0, 4.0, -1.0),
                SigElement::new(4.0, 5.0, 1.0),
                SigElement::new(5.0, 6.0, 1.0),
            ]
        ),
    ]);

    crate::test_len_case!(test_nrzl_len8_cases: Nrzl::new() => [
        ([0, 0, 1, 1, 0, 0, 1, 1],
            [
                SigElement::new(0.0, 1.0, -1.0),
                SigElement::new(1.0, 2.0, -1.0),
                SigElement::new(2.0, 3.0, 1.0),
                SigElement::new(3.0, 4.0, 1.0),
                SigElement::new(4.0, 5.0, -1.0),
                SigElement::new(5.0, 6.0, -1.0),
                SigElement::new(6.0, 7.0, 1.0),
                SigElement::new(7.0, 8.0, 1.0),
            ]
        ),
        ([0, 1, 0, 1, 0, 1, 1, 1],
            [
                SigElement::new(0.0, 1.0, -1.0),
                SigElement::new(1.0, 2.0, 1.0),
                SigElement::new(2.0, 3.0, -1.0),
                SigElement::new(3.0, 4.0, 1.0),
                SigElement::new(4.0, 5.0, -1.0),
                SigElement::new(5.0, 6.0, 1.0),
                SigElement::new(6.0, 7.0, 1.0),
                SigElement::new(7.0, 8.0, 1.0),
            ]
        ),
    ]);

    crate::test_len_case!(test_nrzi_len8_cases: Nrzi::new() => [
        ([0, 0, 1, 1, 0, 0, 1, 1],
            [
                SigElement::new(0.0, 1.0, 1.0),
                SigElement::new(1.0, 2.0, 1.0),
                SigElement::new(2.0, 3.0, -1.0),
                SigElement::new(3.0, 4.0, 1.0),
                SigElement::new(4.0, 5.0, 1.0),
                SigElement::new(5.0, 6.0, 1.0),
                SigElement::new(6.0, 7.0, -1.0),
                SigElement::new(7.0, 8.0, 1.0),
            ]
        ),
        ([0, 1, 0, 1, 0, 1, 1, 1],
            [
                SigElement::new(0.0, 1.0, 1.0),
                SigElement::new(1.0, 2.0, -1.0),
                SigElement::new(2.0, 3.0, -1.0),
                SigElement::new(3.0, 4.0, 1.0),
                SigElement::new(4.0, 5.0, 1.0),
                SigElement::new(5.0, 6.0, -1.0),
                SigElement::new(6.0, 7.0, 1.0),
                SigElement::new(7.0, 8.0, -1.0),
            ]
        ),
    ]);

    #[test]
    fn test_len9_edge_patterns() {
        let seq = [1, 1, 1, 0, 0, 0, 1, 1, 1];
        let exp_l = [
            SigElement::new(0.0, 1.0, 1.0),
            SigElement::new(1.0, 2.0, 1.0),
            SigElement::new(2.0, 3.0, 1.0),
            SigElement::new(3.0, 4.0, -1.0),
            SigElement::new(4.0, 5.0, -1.0),
            SigElement::new(5.0, 6.0, -1.0),
            SigElement::new(6.0, 7.0, 1.0),
            SigElement::new(7.0, 8.0, 1.0),
            SigElement::new(8.0, 9.0, 1.0),
        ];
        let exp_i = [
            SigElement::new(0.0, 1.0, -1.0),
            SigElement::new(1.0, 2.0, 1.0),
            SigElement::new(2.0, 3.0, -1.0),
            SigElement::new(3.0, 4.0, -1.0),
            SigElement::new(4.0, 5.0, -1.0),
            SigElement::new(5.0, 6.0, -1.0),
            SigElement::new(6.0, 7.0, 1.0),
            SigElement::new(7.0, 8.0, -1.0),
            SigElement::new(8.0, 9.0, 1.0),
        ];

        let nrzl = Nrzl::new();
        let nrzi = Nrzi::new();

        assert_eq!(nrzl.encode(&seq).as_ref(), &exp_l);
        assert_eq!(nrzi.encode(&seq).as_ref(), &exp_i);
    }

    #[test]
    fn test_unarios_len1() {
        let nrzl = Nrzl::new();
        let nrzi = Nrzi::new();

        let s0 = [0u8; 1];
        let e0 = [SigElement::new(0.0, 1.0, -1.0)];
        let e1 = [SigElement::new(0.0, 1.0, 1.0)];
        assert_eq!(nrzl.encode(&s0).as_ref(), &e0);
        assert_eq!(nrzi.encode(&s0).as_ref(), &e1);

        let s1 = [1u8; 1];
        let e1_l = [SigElement::new(0.0, 1.0, 1.0)];
        let e1_i = [SigElement::new(0.0, 1.0, -1.0)];
        assert_eq!(nrzl.encode(&s1).as_ref(), &e1_l);
        assert_eq!(nrzi.encode(&s1).as_ref(), &e1_i);
    }
}
