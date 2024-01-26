/**  #| # Baby SNARK (do do dodo dodo)
#| _A simple but expressive SNARK._
#|
#| This is a self-contained development of SNARKs for NP. It is based on
#| [Square Span Program SNARKs](https://eprint.iacr.org/2014/718) by Danezis et al.,
#| which are expressive enough to encode boolean circuits.
#|
#| For detail about the algorithm, especially its security definition and soundness,
#| proof, see [the acompanying paper](https://github.com/initc3/babySNARK/blob/master/babysnark.pdf)
#|
#| This implementation in this file is optimized for readability and simplicity, not
#| performance. The result is that the proof is succinct, but the computation overhead
#| of `Setup` and `Prove` is quadratic in the circuit size, rather than quasilinear.
#| In `babysnark_opt.py` you can find a quasilinear implementation.

#| ## Polynomials library
#| We start with a library, `finite_field/`, for polynomials over finite fields,
#| represented by coefficients. The library includes:
#|  - Constructing a polynomial from a list of coefficients
#|  - Addition, scaling, multiplication of polynomials
#|  - Euclidean division of polynomials
#|  - Lagrange interpolation of polynomials
#|
#| The library is adapted from tutorials by Jeremy Kun.
#| See [A Programmer's Introudction to Mathematics](https://github.com/pim-book/programmers-introduction-to-mathematics)
#| and [Programming with Finite Fields](https://jeremykun.com/2014/03/13/programming-with-finite-fields/)
*/

use finitefields::*;

