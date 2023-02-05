use plotters::prelude::*;
use image::{imageops::FilterType, ImageFormat};
use std::fs::File;
use std::io::BufReader;
use std::time::{SystemTime, UNIX_EPOCH};

fn b_spline(points: &[(f64, f64)], t: f64) -> (f64, f64) {
    //setting variables to represent numerator (sum i=1 -> 6 L_dimension(i)h(i,t)) and denominator (sum i=1 -> 6 h(i,t))
    //where L_dimension(i) is the individual control points where dimension can be x or y
    //where h(i, t) is our scaling factor, which I have chosen to be 4.5^(-(i - (6 + 1)*t)^2)
    //Yes this may be an arguably extremely innefficient way of doing this, however it works and I don't wanna think about it anymore because this is my second day using Rust. Ratio.
    let mut numerx = 0.0;
    let mut denomx = 0.0;
    let mut numery = 0.0;
    let mut denomy = 0.0;
    let mut i = 1.0;
    for elem in points {
        let h_exponent = -(i - (6.0 + 1.0) * t)*(i - (6.0 + 1.0) * t);
        let base = 4.5f64;
        let h = base.powf(h_exponent);
        numerx = numerx + (elem.0 * h);
        denomx = denomx + h;
        numery = numery + (elem.1 * h);
        denomy = denomy + h;
        i = i + 1.0;
    }
    let x = numerx / denomx;
    let y = numery / denomy;
    return(x, y);
}

fn get_divsions_count(puzzle_pieces: f64) -> (f64, f64){
    let mut test_num = puzzle_pieces.powf(0.5f64).floor();
    while puzzle_pieces % test_num != 0f64 {
        test_num-=1f64;
    }
    if test_num < (puzzle_pieces / test_num) {
    (test_num - 1f64, (puzzle_pieces / test_num)- 1f64)
    }
    else {
        ((puzzle_pieces / test_num)- 1f64 ,test_num - 1f64)
    }
}


//takes a distance 'dist', and returns a vector of the evenly spaced 'subdivisions' numbers between 0 and that distance then centers around dist
fn subdivide(dist: f64, subdivisions: f64) -> Vec<f64> {
    let mut v = vec![];
    if subdivisions != 0.0 {
        let step = dist / (subdivisions + 1.0);
        let mut cur = 0.0;
        let sdv = subdivisions as i32;
        v.push(0f64);
        for _i in 0..sdv {
            cur += step;
            v.push(cur);
        }
        v.push(dist);
        for elem in v.iter_mut() {
            *elem -= dist / 2f64;
    }
    }
    v
}

//takes in vectors of subdivisions in x and y and returns a tuple of vectors holding start and end control points
fn cbounds(x_divs: Vec<f64>, y_divs: Vec<f64>) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
    let mut vx = Vec::new();
    for y in 1..y_divs.len()-1 {
    vx.push((x_divs[0], y_divs[y]));
        for x in 1..x_divs.len()-1 {
            vx.push((x_divs[x], y_divs[y]));
            vx.push((x_divs[x], y_divs[y]));
        }
        vx.push((x_divs[x_divs.len()-1], y_divs[y]));
    }
    
    let mut vy = Vec::new();
    for x in 1..x_divs.len()-1 {
    vy.push((x_divs[x], y_divs[0]));
        for y in 1..y_divs.len()-1 {
            vy.push((x_divs[x], y_divs[y]));
            vy.push((x_divs[x], y_divs[y]));
        }
        vy.push((x_divs[x], y_divs[y_divs.len()-1]));
    }
    
    (vx,vy)
}

fn horiz_bspline_cpoints(start_point: (f64, f64), end_point: (f64, f64)) -> Vec<(f64, f64)> {
    let mut cpoints = Vec::new();
    let start = SystemTime::now();
    let seed = start.duration_since(UNIX_EPOCH).unwrap();
    let num = ((seed.as_secs_f64()*10000.0 % 10.0 / 10.0)*100.0).round() / 100.0;
    let flip;
    if num < 0.5{
        flip = 1.0;
    }
    else{
        flip = -1.0;
    }
    cpoints.push(start_point);
    let step = flip * 0.25 * ((end_point.0 - start_point.0).powf(2.0) + (end_point.1 - start_point.1).powf(2.0)).powf(0.5);
    let middle = ((start_point.0 + end_point.0)/2.0 + step*num*0.25, (start_point.1 + end_point.1)/2.0 + step*num*0.25);
    cpoints.push(middle);
        //randomize 4 of these later
    let first = (middle.0 - step.abs(), middle.1 + step);
    let second = (middle.0 + step.abs(), middle.1 + step);
    cpoints.push(first);
    cpoints.push(second);
    cpoints.push(middle);
    cpoints.push(end_point);
    cpoints
}

fn vert_bspline_cpoints(start_point: (f64, f64), end_point: (f64, f64)) -> Vec<(f64, f64)> {
    let mut cpoints = Vec::new();
    let start = SystemTime::now();
    let seed = start.duration_since(UNIX_EPOCH).unwrap();
    let num = ((seed.as_secs_f64()*10000.0 % 10.0 / 10.0)*100.0).round() / 100.0;
    let flip;
    if num < 0.5{
        flip = 1.0;
    }
    else{
        flip = -1.0;
    }
    cpoints.push(start_point);
    let step = flip* 0.25 * ((end_point.0 - start_point.0).powf(2.0) + (end_point.1 - start_point.1).powf(2.0)).powf(0.5);
    let middle = ((start_point.0 + end_point.0)/2.0 + num*step*0.25, (start_point.1 + end_point.1)/2.0 + num*step*0.25);
    cpoints.push(middle);
    //randomize 4 of these later
    let first = (middle.0 + step, middle.1 - step.abs());
    let second = (middle.0 + step, middle.1 + step.abs());
    cpoints.push(first);
    cpoints.push(second);
    cpoints.push(middle);
    cpoints.push(end_point);
    cpoints
}



//Calculate the points on arbitrary B-spline curve using the b_spline function
fn splinerx0v0(points: Vec<(f64, f64)>) -> Vec<(f64, f64)>{
    let mut data = vec![];
    for t in 0..100 {
        let x = b_spline(&points, t as f64 / 100.0);
        data.push(x);
    }
    data
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the control points for the B-spline curve
    //let points = vert_bspline_cpoints((0.0, 2.0), (0.0, -2.0));
    //let points2 = horiz_bspline_cpoints((-3.0, -1.0),(3.0, -1.0));
   /* let points = vert_bspline_cpoints((-4.5, 2.0), (-4.5, 4.0));
    let points2 = vert_bspline_cpoints((-3.0, 2.0), (-3.0, 4.0));
    let points3 = vert_bspline_cpoints((-1.5, 2.0), (-1.5, 4.0));
    let points4 = horiz_bspline_cpoints((-3.0, 0.0), (3.0, 0.0));
    let points5 = horiz_bspline_cpoints((-3.0, 1.5), (3.0, 1.5)); */

    // Create a new drawing backend and set the size of the output image
    let root = BitMapBackend::new("spline.png", (1024, 768)).into_drawing_area();

    // Set up the coordinate system for the plot
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("Puzzle 0.0", ("sans-serif", 50).into_font())
        .build_cartesian_2d(-4.0..4.0, -4.0..4.0)
        .expect("Unable to create chart");

        let xratio = 3f64;
        let yratio = 2f64;

        let (w, h) = chart.plotting_area().dim_in_pixel();
        let mut image = image::load(
        BufReader::new(
            File::open("beep.png").map_err(|e| {
                eprintln!("Unable to open file lol.jpg");
                e
            })?),
        ImageFormat::Png,
    )?
     .resize_exact(w - w / 5, h - h / 2, FilterType::Nearest);

    let elem: BitMapElement<_> = ((-(xratio), yratio), image.crop(0,0,2*(xratio as u32)*1024/8, 2*(yratio as u32)*768/8)).into();

    chart.draw_series(std::iter::once(elem))?;
        let pieces = 100f64;
        //prepare to draw a fuarking rectangle (1 rep)        
        let divs = get_divsions_count(pieces);
        let xdivs = subdivide(xratio*2f64, divs.1);
        let ydivs = subdivide(yratio*2f64, divs.0);
        //confused? me too.
        let points = cbounds(xdivs, ydivs);

    for i in (0..points.0.len()-1).step_by(2){
        let graphe = horiz_bspline_cpoints(points.0[i], points.0[i+1]);
        let data = splinerx0v0(graphe);
        chart.draw_series(LineSeries::new(data, &BLACK))?;
    }
    for j in (0..points.1.len()-1).step_by(2){
        let graphe = vert_bspline_cpoints(points.1[j], points.1[j+1]);
        let data2 = splinerx0v0(graphe);
        chart.draw_series(LineSeries::new(data2, &BLACK))?;
    } 
    //drawing the rectangle (fuuuuuuuuark)
    let rectangle = vec![(-1.0*xratio, -1.0*yratio), (1.0*xratio, -1.0*yratio), (1.0*xratio, 1.0*yratio), (-1.0*xratio, 1.0*yratio), (-1.0*xratio, -1.0*yratio)];
    chart.draw_series(std::iter::once(PathElement::new(rectangle.clone(), &BLACK)))?;
    // Save the chart to a file
    root.present()?;
    Ok(())
}
