// Compiler Backend: Cayley Homomorphism
#![no_std]

/// Finite field arithmetic over GF(p)
/// Auto-generated from Topology Model: crypto-groups

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct FieldElement {
    pub value: [u64; 4], // 256-bit element
}

impl FieldElement {
    #[inline(always)]
    pub fn add(&self, other: &Self) -> Self {
        let mut res = [0u64; 4];
        let mut carry = 0u64;
        
        unsafe {
            for i in 0..4 {
                let (sum, c1) = self.value[i].overflowing_add(other.value[i]);
                let (sum, c2) = sum.overflowing_add(carry);
                res[i] = sum;
                carry = (c1 | c2) as u64;
            }
        }
        
        FieldElement { value: res }
    }
}