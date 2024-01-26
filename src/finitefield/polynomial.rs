
/* # create a polynomial with coefficients in a field; coefficients are in
# increasing order of monomial degree so that, for example, [1,2,3]
# corresponds to 1 + 2x + 3x^2 */

fn polynomialsOver(coeff : &[i32]) {

}

struct  Polynomial<T : Ord>{
    field : T,
    degree : usize,
    coefficients : Vec<i32>

}
impl<T : Ord> Polynomial<T>{
    fn is_zero(&self) -> bool{
        self.coefficients.is_empty()
    }

    fn len(&self) -> usize{
        self.coefficients.len()
    }
    fn sub(mut self, other : Polynomial<T>) -> Polynomial<T>{
        if self.degree == other.degree && self.coefficients.len() ==other.coefficients.len(){
            for i in 0..self.coefficients.len(){
                self.coefficients[i] -=other.coefficients[i];
            }
        }
        self
    }
    fn add(mut self, other : Polynomial<T>) -> Polynomial<T>{
        if self.degree == other.degree && self.coefficients.len() ==other.coefficients.len(){
            for i in 0..self.coefficients.len(){
                self.coefficients[i] +=other.coefficients[i];
            }
        }
        self
    }
    /*
     def leadingCoefficient(self): return self.coefficients[-1]
        def degree(self): return abs(self) - 1
 */
fn leadingCoefficient(&self) -> i32 {
    self.coefficients[self.coefficients.len()]
}
fn degree(&self) -> usize {
    if self.coefficients.is_empty() || self.coefficients.len() == 1{
        return 0
    }
    self.len() - 1
}
}
