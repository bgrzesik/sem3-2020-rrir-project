
use crate::base_function::BaseFunction;
use crate::integration::integrate;


pub trait Problem {
    fn range(&self) -> std::ops::Range<f64>;

    fn left_zeros(&self) -> bool { true }
    fn right_zeros(&self) -> bool { true }

    fn left_integral<BFn: BaseFunction>(&self, x: f64, u: &BFn, v: &BFn) -> f64;
    fn free_left_terms<BFn: BaseFunction>(&self, u: &BFn, v: &BFn) -> f64;

    fn right_integral<BFn: BaseFunction>(&self, x: f64, v: &BFn) -> f64;
    fn free_right_terms<BFn: BaseFunction>(&self, v: &BFn) -> f64;
}

pub struct ComputedFunction<BFn: BaseFunction> {
    bases: Vec<BFn>,
    scalars: na::DVector<f64>,
}

impl <BFn: BaseFunction> ComputedFunction<BFn> {

    pub fn get_bases<P>(problem: &P, n: usize) -> Vec<BFn>
        where P: Problem
    {
        let mut bases = Vec::<BFn>::with_capacity(n + 1);

        if !problem.left_zeros() {
            bases.push(BFn::new(0, n, problem.range()))
        }
        
        for i in 1..n {
            bases.push(BFn::new(i, n, problem.range()));
        }

        if !problem.right_zeros() {
            bases.push(BFn::new(n, n, problem.range()))
        }

        return bases;
    }

    pub fn find_solution<P>(problem: &P, n: usize) -> Self
        where 
            P: Problem
    {
        let bases = Self::get_bases(problem, n);
        let mut left = na::DMatrix::<f64>::zeros(bases.len(), bases.len());
        let mut right = na::DVector::<f64>::zeros(bases.len());

        for row in 0..bases.len() {
            let v = &bases[row];

            let right = &mut right[row];

            
            let func = |x| problem.right_integral(x, v);
            let integral = integrate(func, problem.range());

            *right = problem.free_right_terms(v) + integral;

            for col in 0..bases.len() {
                let u = &bases[col];
                let left = &mut left[(row, col)];

                let func = |x| problem.left_integral(x, u, v);
                let integral = integrate(func, problem.range());

                *left = problem.free_left_terms(u, v) +  integral;
            }
        }
        
        println!("{:?}", right);
        println!("{:?}", left);
        
        let solution = left.lu().solve(&right);

        println!("{:?}", solution);

        let solution = solution.unwrap();

        Self { bases, scalars: solution }
    }


    pub fn evalute(&self, x: f64) -> f64 {
        self.bases
            .iter()
            .zip(self.scalars.iter())
            .map(|(ref e, w)| e.regular(x) * w)
            .sum()
    }

}

pub struct MaterialVibration;

impl Problem for MaterialVibration {

    fn range(&self) -> std::ops::Range<f64> { 0.0f64..2.0f64 }

    fn left_zeros(&self) -> bool { true }

    fn right_zeros(&self) -> bool { false }

    fn left_integral<BFn: BaseFunction>(&self, x: f64, u: &BFn, v: &BFn) -> f64 {
        u.derivative(x) * v.derivative(x) - u.regular(x) * v.regular(x)
    }

    fn free_left_terms<BFn: BaseFunction>(&self, u: &BFn, v: &BFn) -> f64 {
        - u.regular(2f64) * v.regular(2f64)
    }

    fn right_integral<BFn: BaseFunction>(&self, x: f64, v: &BFn) -> f64 {
        x.sin() * v.regular(x)
    }

    fn free_right_terms<BFn: BaseFunction>(&self, _: &BFn) -> f64 {
        0f64
    }
}