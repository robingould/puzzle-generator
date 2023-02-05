# puzzle_generator
generate 3 by 2 puzzles in rust with any image

put however many puzzles pieces u want on line 193 (let pieces = {number of pieces u want}f64)

Basically just divides up the piece count into the middlemost factors, and subdivides the dimensions of the puzzle by the factors. Uses those subdivisions to make b spline curves based on start and endpoints with slight variations. I don't know why I felt like not using the random crate, but I made a pseudorandom number generator to pick the variance.

this is the first rust thing I've ever done besides hello world so please be nice 

![image](https://user-images.githubusercontent.com/13643473/216840045-87a9e1f1-14c1-432d-bd1c-4e6fdc0ca1e1.png)
