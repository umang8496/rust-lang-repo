// main.rs

#[derive(Debug)]
struct Triangle {
    side_ab: i32,
    side_bc: i32,
    side_ca: i32,
}

impl Triangle {
    fn compute_area(&self) -> f32 {
        let s = (self.side_ab + self.side_bc + self.side_ca) / 2;
        let area = ((s * (s - self.side_ab) * (s - self.side_bc) * (s - self.side_ca)) as f32).sqrt();
        return area
    }
}

impl Triangle {
    fn compute_perimeter(&self) -> i32 {
        let perimeter = self.side_ab + self.side_bc + self.side_ca;
        return perimeter
    }
}

fn main() {
    // let triangle = Triangle {
    //     side_ab: 4,
    //     side_bc: 3,
    //     side_ca: 5,
    // };

    // println!("triangle: {}", triangle);
    // `Triangle` doesn't implement `std::fmt::Display`
    // `Triangle` cannot be formatted with the default formatter
    // help: the trait `std::fmt::Display` is not implemented for `Triangle`


    // println!("triangle: {:?}", triangle);
    // triangle: Triangle { side_ab: 4, side_bc: 3, side_ca: 5 }
    // if we didn't have #[derive(Debug)] then the above statement would have failed 
    // `Triangle` doesn't implement `Debug`
    // `Triangle` cannot be formatted using `{:?}`
    // help: the trait `Debug` is not implemented for `Triangle`


    // println!("triangle: {:#?}", triangle);
    // triangle: Triangle {
    //     side_ab: 4,
    //     side_bc: 3,
    //     side_ca: 5,
    // }

    // // println!("Area of triangle: {}", triangle.compute_area());

    // // println!("Perimeter of triangle: {}", triangle.compute_perimeter());

    // println!("Area of triangle is: {}", compute_area(&triangle));

}

fn compute_area(triangle: &Triangle) -> f32 {
    let s = (triangle.side_ab + triangle.side_bc + triangle.side_ca) / 2;
    let area = ((s * (s - triangle.side_ab) * (s - triangle.side_bc) * (s - triangle.side_ca)) as f32).sqrt();
    return area;
}


