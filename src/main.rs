fn cubic_bezier_interpolation(x2:f64,y2:f64,x1:f64,y1:f64,x0:f64,cpts:[f64;4]){
    //
    // control points are used to 
    // relate the (x = f(t), y = f(t) )
    // have to calculate the inverse t = finv(x)
    // and find y for the t.
}

fn main() {
    let x = 0.5; // current_value of x
    let cpts: [f64;4] = [2.0,2.0,3.5,4.0]; // control_points
                                           
    // points that are used for reference base for interpolation
    let x2 = 0.0;
    let x1 = 3.0;
    let y2 = 0.0;
    let y1 = 4.0;
}

