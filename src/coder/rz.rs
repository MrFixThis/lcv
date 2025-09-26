use super::{LineCoder, SigElement};

#[derive(Debug, Clone, Copy)]
pub struct Rz {
    tb: f64,
    v: f64,
}

impl Default for Rz {
    fn default() -> Self {
        Self {
            tb: super::GLOB_BASE_TB,
            v: super::GLOB_BASE_V,
        }
    }
}

impl Rz {
    const DEF_DUTY: f64 = 0.5;

    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

impl LineCoder for Rz {
    fn encode(&self, bits: &[u8]) -> Box<[SigElement]> {
        let h = self.tb * Self::DEF_DUTY;
        let mut t = 0.0;
        let mut out = Vec::new();

        for &bit in bits {
            let tf0 = t + h;
            if tf0 > t {
                out.push(SigElement::new(
                    t,
                    tf0,
                    if bit == 1 { self.v } else { -self.v },
                ));
            }

            let tf1 = t + self.tb;
            if tf1 > tf0 {
                out.push(SigElement::new(tf0, tf1, 0.0));
            }

            t += self.tb;
        }

        out.into_boxed_slice()
    }
}

#[cfg(test)]
mod tests {
    use crate::coder::{LineCoder, SigElement, rz::Rz};

    crate::test_len_case!(test_rz_len4_cases: Rz::new() => [
        ([0,0,0,0], [
            SigElement::new(0.0,0.5,-1.0),
            SigElement::new(0.5,1.0,0.0),
            SigElement::new(1.0,1.5,-1.0),
            SigElement::new(1.5,2.0,0.0),
            SigElement::new(2.0,2.5,-1.0),
            SigElement::new(2.5,3.0,0.0),
            SigElement::new(3.0,3.5,-1.0),
            SigElement::new(3.5,4.0,0.0),
        ]),
        ([1,1,1,1], [
            SigElement::new(0.0,0.5,1.0),
            SigElement::new(0.5,1.0,0.0),
            SigElement::new(1.0,1.5,1.0),
            SigElement::new(1.5,2.0,0.0),
            SigElement::new(2.0,2.5,1.0),
            SigElement::new(2.5,3.0,0.0),
            SigElement::new(3.0,3.5,1.0),
            SigElement::new(3.5,4.0,0.0),
        ]),
        ([1,0,1,0], [
            SigElement::new(0.0,0.5,1.0),
            SigElement::new(0.5,1.0,0.0),
            SigElement::new(1.0,1.5,-1.0),
            SigElement::new(1.5,2.0,0.0),
            SigElement::new(2.0,2.5,1.0),
            SigElement::new(2.5,3.0,0.0),
            SigElement::new(3.0,3.5,-1.0),
            SigElement::new(3.5,4.0,0.0),
        ]),
    ]);

    crate::test_len_case!(test_rz_len6_cases: Rz::new() => [
        ([0,1,1,1,1,0], [
            SigElement::new(0.0,0.5,-1.0),
            SigElement::new(0.5,1.0,0.0),
            SigElement::new(1.0,1.5,1.0),
            SigElement::new(1.5,2.0,0.0),
            SigElement::new(2.0,2.5,1.0),
            SigElement::new(2.5,3.0,0.0),
            SigElement::new(3.0,3.5,1.0),
            SigElement::new(3.5,4.0,0.0),
            SigElement::new(4.0,4.5,1.0),
            SigElement::new(4.5,5.0,0.0),
            SigElement::new(5.0,5.5,-1.0),
            SigElement::new(5.5,6.0,0.0),
        ]),
        ([1,0,1,0,1,0], [
            SigElement::new(0.0,0.5,1.0),
            SigElement::new(0.5,1.0,0.0),
            SigElement::new(1.0,1.5,-1.0),
            SigElement::new(1.5,2.0,0.0),
            SigElement::new(2.0,2.5,1.0),
            SigElement::new(2.5,3.0,0.0),
            SigElement::new(3.0,3.5,-1.0),
            SigElement::new(3.5,4.0,0.0),
            SigElement::new(4.0,4.5,1.0),
            SigElement::new(4.5,5.0,0.0),
            SigElement::new(5.0,5.5,-1.0),
            SigElement::new(5.5,6.0,0.0),
        ]),
    ]);

    #[test]
    fn test_rz_unaries_len1() {
        let enc = Rz::new();

        let s0 = [0u8; 1];
        let e0 = [
            SigElement::new(0.0, 0.5, -1.0),
            SigElement::new(0.5, 1.0, 0.0),
        ];
        assert_eq!(enc.encode(&s0).as_ref(), &e0);

        let s1 = [1u8; 1];
        let e1 = [
            SigElement::new(0.0, 0.5, 1.0),
            SigElement::new(0.5, 1.0, 0.0),
        ];
        assert_eq!(enc.encode(&s1).as_ref(), &e1);
    }
}
