
# Structs in Rust  

---

A `struct` is a custom data type that lets us club different atomic or types together which then constituets some meaningful object.  
It is somewhat like an object's data attributes, certainly without any methods or functions.  

---

## Define & Instantiate Structs  

Let's say we want to define/represent a triangle object and then compute its are and perimeter.
Since a triangle has 3 sides, we can do something like the following:

```rust
    fn main() {
        let side_ab: i32 = 4;
        let side_bc: i32 = 3;
        let side_ca: i32 = 5;

        println!("Area of triangle is: {}", compute_area(side_ab, side_bc, side_ca));
    }

    fn compute_area(ab: i32, bc: i32, ca: i32) -> f32 {
        let s = (ab + bc + ca) / 2;
        let area = ((s * (s - ab) * (s - bc) * (s - ca)) as f32).sqrt();
        return area;
    }
```

The program would work and compute the area precisely but there is a huge logical inconsistency as well.  
The variables `side_ab`, `side_bc` and `side_ca` exist indenpendent of each other, even though they are part of a triangle, they donot represent that.  

In order to address this very situtaion, Rust provides us with something called `struct`, which is collection of associated attributes.  
Lets rewrite the earlier program differently using `struct`

```rust
    #[derive(Debug)]
    struct Triangle {
        side_ab: i32,
        side_bc: i32,
        side_ca: i32
    }

    fn main() {
        let triangle = Triangle {
            side_ab: 4,
            side_bc: 3,
            side_ca: 5
        };

        println!("Area of triangle is: {}", compute_area(&triangle));
    }

    fn compute_area(triangle: &Triangle) -> f32 {
        let s = (triangle.side_ab + triangle.side_bc + triangle.side_ca) / 2;
        let area = ((s * (s - triangle.side_ab) * (s - triangle.side_bc) * (s - triangle.side_ca)) as f32).sqrt();
        return area;
    }
```

This approach is better than the previous one.  
Here, the information about the triangle is all tied up in a `struct`.  
Like the other objects, the `struct` objects can also be borrowed or passed as a reference to other funtions.  

But still, there is one problem.  
The funtion `fn compute_area(triangle: &Triangle) -> f32` is still not tied to the `Triangle` type, rather it is quite open to be accessed by any other type as well.  
This may have some problems for a fairly larger program.  

But Rust has something to address this situation as well.  
We can define methods specifically for the `Triangle` type and closely tie up the behaviour to a `struct`.  
Lets once again write the same program but with yet another approach.

```rust
    #[derive(Debug)]
    struct Triangle {
        side_ab: i32,
        side_bc: i32,
        side_ca: i32
    }

    impl Triangle {
        fn compute_area(&self) -> f32 {
            let s = (self.side_ab + self.side_bc + self.side_ca) / 2;
            let area = ((s * (s - self.side_ab) * (s - self.side_bc) * (s - self.side_ca)) as f32).sqrt();
            return area
        }
    }

    fn main() {
        let triangle = Triangle {
            side_ab: 4,
            side_bc: 3,
            side_ca: 5
        };

        println!("Area of triangle is: {}", triangle.compute_area());
    }
```

So using the `impl` blocks, we can write behvaiour for a specific `struct` type.  
We can multiple such `impl` blocks for a single `struct` type, but we should have as mush less as possible.  

---
