TODO: make a gui
TODO: abstract puzzle size  

# puzzle_generator
generate 3 by 2 puzzles in rust with any image using plotters and image crates

put the amount of puzzles pieces you want on line 193 (let pieces = {number of pieces you want}f64)

Basically just divides up the piece count into the middlemost factors, and subdivides the dimensions of the puzzle by the factors. Uses those subdivisions to make b spline curves based on start and endpoints with slight variations.

this is the first rust thing I've ever done besides hello world so please be nice 

![image](https://user-images.githubusercontent.com/13643473/216840045-87a9e1f1-14c1-432d-bd1c-4e6fdc0ca1e1.png)
