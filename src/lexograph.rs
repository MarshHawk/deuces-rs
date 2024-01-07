/*Suppose we have a pattern of N bits set to 1 in an integer and we want the next permutation of N 1 bits in a lexicographical sense. 
For example, if N is 3 and the bit pattern is 00010011, the next patterns would be 00010101, 00010110, 00011001,00011010, 00011100, 00100011, and so forth. The following is a fast way to compute the next permutation.
unsigned int v; // current permutation of bits 
unsigned int w; // next permutation of bits

unsigned int t = v | (v - 1); // t gets v's least significant 0 bits set to 1
// Next set to 1 the most significant bit to change, 
// set to 0 the least significant ones, and add the necessary 1 bits.
w = (t + 1) | (((~t & -~t) - 1) >> (__builtin_ctz(v) + 1));  */
fn next_permutation(mut v: u32) -> impl Iterator<Item = u32> {
    std::iter::from_fn(move || {
        let t = v | (v - 1);
        let w = (t + 1) | (((!t & -!t) - 1) >> v.trailing_zeros() + 1);
        std::mem::swap(&mut v, &mut w);
        Some(w)
    })
}


