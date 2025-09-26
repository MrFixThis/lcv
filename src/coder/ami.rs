use super::{LineCoder, SigElement};

#[derive(Debug, Clone, Copy)]
pub struct Ami {
    tb: f64,
    v: f64,
}

impl Default for Ami {
    fn default() -> Self {
        Self {
            tb: super::GLOB_BASE_TB,
            v: -super::GLOB_BASE_V,
        }
    }
}

impl Ami {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

impl LineCoder for Ami {
    fn encode(&self, bits: &[u8]) -> Box<[SigElement]> {
        let mut t = 0.0;
        let mut lvl = self.v;

        bits.iter()
            .filter_map(|&bit| {
                let mut res = None;
                let tf = t + self.tb;
                if bit == 1 {
                    lvl = -lvl;
                    if tf > t {
                        res = Some(SigElement::new(t, tf, lvl));
                    }
                } else if tf > t {
                    res = Some(SigElement::new(t, tf, 0.0));
                }

                t += self.tb;
                res
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::coder::{LineCoder, SigElement, ami::Ami};

    crate::test_len_case!(test_ami_len4_cases: Ami::new() => [
        ([0,0,0,0], [
            SigElement::new(0.0,1.0, 0.0),
            SigElement::new(1.0,2.0, 0.0),
            SigElement::new(2.0,3.0, 0.0),
            SigElement::new(3.0,4.0, 0.0),
        ]),
        ([1,1,1,1], [
            SigElement::new(0.0,1.0, 1.0),
            SigElement::new(1.0,2.0,-1.0),
            SigElement::new(2.0,3.0, 1.0),
            SigElement::new(3.0,4.0,-1.0),
        ]),
        ([1,0,1,0], [
            SigElement::new(0.0,1.0, 1.0),
            SigElement::new(1.0,2.0, 0.0),
            SigElement::new(2.0,3.0,-1.0),
            SigElement::new(3.0,4.0, 0.0),
        ]),
    ]);

    crate::test_len_case!(test_ami_len6_cases: Ami::new() => [
        ([0,1,1,1,1,0], [
            SigElement::new(0.0,1.0, 0.0),
            SigElement::new(1.0,2.0, 1.0),
            SigElement::new(2.0,3.0,-1.0),
            SigElement::new(3.0,4.0, 1.0),
            SigElement::new(4.0,5.0,-1.0),
            SigElement::new(5.0,6.0, 0.0),
        ]),
        ([1,0,1,0,1,0], [
            SigElement::new(0.0,1.0, 1.0),
            SigElement::new(1.0,2.0, 0.0),
            SigElement::new(2.0,3.0,-1.0),
            SigElement::new(3.0,4.0, 0.0),
            SigElement::new(4.0,5.0, 1.0),
            SigElement::new(5.0,6.0, 0.0),
        ]),
    ]);

    crate::test_len_case!(test_ami_len8_cases: Ami::new() => [
        ([0,0,1,1,0,0,1,1], [
            SigElement::new(0.0,1.0, 0.0),
            SigElement::new(1.0,2.0, 0.0),
            SigElement::new(2.0,3.0, 1.0),
            SigElement::new(3.0,4.0,-1.0),
            SigElement::new(4.0,5.0, 0.0),
            SigElement::new(5.0,6.0, 0.0),
            SigElement::new(6.0,7.0, 1.0),
            SigElement::new(7.0,8.0,-1.0),
        ]),
        ([0,1,0,1,0,1,1,1], [
            SigElement::new(0.0,1.0, 0.0),
            SigElement::new(1.0,2.0, 1.0),
            SigElement::new(2.0,3.0, 0.0),
            SigElement::new(3.0,4.0,-1.0),
            SigElement::new(4.0,5.0, 0.0),
            SigElement::new(5.0,6.0, 1.0),
            SigElement::new(6.0,7.0,-1.0),
            SigElement::new(7.0,8.0, 1.0),
        ]),
    ]);

    #[test]
    fn test_ami_runs_of_zeroes() {
        let seq = [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1];
        let exp = [
            SigElement::new(0.0, 1.0, 0.0),
            SigElement::new(1.0, 2.0, 0.0),
            SigElement::new(2.0, 3.0, 0.0),
            SigElement::new(3.0, 4.0, 1.0),
            SigElement::new(4.0, 5.0, 0.0),
            SigElement::new(5.0, 6.0, 0.0),
            SigElement::new(6.0, 7.0, 0.0),
            SigElement::new(7.0, 8.0, -1.0),
            SigElement::new(8.0, 9.0, 0.0),
            SigElement::new(9.0, 10.0, 0.0),
            SigElement::new(10.0, 11.0, 0.0),
            SigElement::new(11.0, 12.0, 1.0),
        ];

        let enc = Ami::new();
        assert_eq!(enc.encode(&seq).as_ref(), &exp);
    }

    #[test]
    fn test_ami_unarios_len1() {
        let enc = Ami::new();

        let s0 = [0u8; 1];
        let e0 = [SigElement::new(0.0, 1.0, 0.0)];
        assert_eq!(enc.encode(&s0).as_ref(), &e0);

        let s1 = [1u8; 1];
        let e1 = [SigElement::new(0.0, 1.0, 1.0)];
        assert_eq!(enc.encode(&s1).as_ref(), &e1);
    }
}
