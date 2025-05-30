use russell_lab::*;
use russell_sparse::prelude::*;

#[test]
fn test_complex_umfpack() -> Result<(), StrError> {
    let n = 10;
    let d = (n as f64) / 10.0;
    let mut coo = ComplexCooMatrix::new(n, n, n, Sym::No)?;
    let mut x_correct = ComplexVector::new(n);
    let mut rhs = ComplexVector::new(n);
    for k in 0..n {
        // put diagonal entries
        let akk = cpx!(10.0 + (k as f64) * d, 10.0 - (k as f64) * d);
        coo.put(k, k, akk)?;
        // let the exact solution be k + 0.5i
        x_correct[k] = cpx!(k as f64, 0.5);
        // generate RHS to match solution
        rhs[k] = akk * x_correct[k];
    }
    // println!("a =\n{}", coo.as_dense());
    // println!("x =\n{}", x_correct);
    // println!("b =\n{}", rhs);
    let mut x = ComplexVector::new(n);
    let mut solver = ComplexSolverUMFPACK::new()?;
    solver.factorize(&coo, None)?;
    solver.solve(&mut x, &rhs, false)?;
    complex_vec_approx_eq(&x, &x_correct, 1e-14);
    Ok(())
}
