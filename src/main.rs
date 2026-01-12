use eqsolver::single_variable::FDNewton;
use nalgebra::{vector, Vector2};



// control points are used to 
// relate the (x = f(t), y = f(t) )
// have to calculate the inverse t = finv(x)
// and find y for the t.
// term1 = (1-t)^3 * (x'0)
// term2 = 3 * (1-t)^2 * t * (x'1)
// term3 = 3 * (1-t) * t^2 * (x'2)
// term4 = t^3 * (x'3)
// x(t) =  term1 + term2 + term3 + term4
//
fn cubic_bezier_interpolation(x2:f64,y2:f64,x1:f64,y1:f64,x0:f64,cpts:[f64;4])->Option<f64>{
    let t = inv_find_t(x0,cpts)?;
    let y0 = 3.*(1.-t).powi(2)*t*y1 + 3.*(1.-t)*t.powi(2)*y2 + t.powi(3);
    Some(y0)
}
fn inv_find_t(x0: f64,cpts: [f64;4])->Option<f64>{

    let x1 = cpts[0];
    let x2 = cpts[2];

    let f = move |t:f64| {
        3.*(1.-t).powi(2)*t*x1 + 3.*(1.-t)*t.powi(2)*x2+ t.powi(3) - x0
    };

    FDNewton::new(f).solve(0.0).map(|e| e.clamp(0.0,1.0) ).ok()
}

fn main() {
    let x = 0.5; // current_value of x
    let cpts: [f64;4] = [2.0,2.0,3.5,4.0]; // control_points
                                           
    // points that are used for reference base for interpolation
    let x2 = 0.0;
    let x1 = 3.0;
    let y2 = 0.0;
    let y1 = 4.0;

    println!("{:?}",cubic_bezier_interpolation(x2,y2,x1,y1,x,cpts));
}

