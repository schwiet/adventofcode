package utils

// Compute GCD using Euclid's algorithm
func gcd(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

// extendedEuclid solves for x,y in: a*x + b*y = gcd(a,b)
func extendedEuclid(a, b int) (int, int, int) {
	if b == 0 {
		return a, 1, 0
	}
	d, x1, y1 := extendedEuclid(b, a%b)
	x := y1
	y := x1 - (a/b)*y1
	return d, x, y
}

// solve two linear Diophantine equations:
// a1*x + b1*y = z1
// a2*x + b2*y = z2
// Implementation from chatgpt: golang implementation for solving systems of two
// Diophantine equations
// Returns whether a solution exists and if so, the solution with maximum y (if defined).
func SolveTwoEquations(a1, b1, z1, a2, b2, z2 int) (exists bool, xSol, ySol int, maxY bool) {
	// determinant
	determinant := a1*b2 - a2*b1

	if determinant != 0 {
		// one unique solution if it exists in integers
		xNum := z1*b2 - z2*b1
		yNum := a1*z2 - a2*z1

		if xNum%determinant == 0 && yNum%determinant == 0 {
			// unique integer solution
			xSol = xNum / determinant
			ySol = yNum / determinant
			// since there's only one solution, it's trivially the maximum y
			return true, xSol, ySol, true
		} else {
			// no integral intersection
			return false, 0, 0, false
		}
	} else {
		// D = 0 means lines are parallel or coincident

		// check if they are coincident
		// for them to be coincident, ratios of a1:a2 and b1:b2 and z1:z2 must match.
		isCoincident := false

		// Check a scenario: If a1 = b1 = 0, then z1 must be 0 for any solution (like 0x+0y=0)
		// Otherwise if one line is effectively "0=0", it's all solutions.
		if a2 == 0 && b2 == 0 && z2 == 0 {
			if a1 == 0 && b1 == 0 && z1 == 0 {
				// Everything is 0=0, infinite solutions
				isCoincident = true
			} else {
				// first line is something like a1x+b1y=z1
				// this line itself has infinite solutions if gcd(a1,b1)|z1.
				d := gcd(a1, b1)
				if d != 0 && z1%d == 0 {
					isCoincident = true
				} else {
					// If not divisible, no solution at all
					return false, 0, 0, false
				}
			}
		} else {
			// at least one of a2 or b2 is nonzero
			// try to find a ratio using a nonzero coefficient from line2
			var baseRatioNum, baseRatioDen int
			if a2 != 0 {
				baseRatioNum = a1
				baseRatioDen = a2
			} else {
				baseRatioNum = b1
				baseRatioDen = b2
			}
			dg := gcd(baseRatioNum, baseRatioDen)
			baseRatioNum /= dg
			baseRatioDen /= dg

			// check b1:b2 ratio
			if b2 != 0 {
				num := b1
				den := b2
				g := gcd(num, den)
				num /= g
				den /= g
				if num != baseRatioNum || den != baseRatioDen {
					// Ratios don't match
					return false, 0, 0, false
				}
			} else {
				// If b2 == 0 but b1 != (aligned with ratio?),
				// if b2=0 then for them to be coincident and a2!=0 was chosen, b1 must also have ratio consistent
				if b1 != 0 {
					return false, 0, 0, false
				}
			}

			// Check z1:z2 ratio
			if z2 != 0 {
				num := z1
				den := z2
				g := gcd(num, den)
				num /= g
				den /= g
				if num != baseRatioNum || den != baseRatioDen {
					// Not coincident
					return false, 0, 0, false
				}
			} else {
				// If z2=0, then z1 must be 0 too if lines are coincident (under the same ratio)
				if z1 != 0 {
					return false, 0, 0, false
				}
			}

			// All ratios match, lines are coincident
			isCoincident = true
		}

		if isCoincident {
			// Infinite solutions. Solve one equation for parametric form.
			// a1x + b1y = z1
			d1 := gcd(a1, b1)
			if d1 == 0 {
				// Equation is 0=0, all x,y are solutions: no maximum y.
				return true, 0, 0, false // maxY=false means no maximum (unbounded).
			}
			if z1%d1 != 0 {
				// No solution actually
				return false, 0, 0, false
			}

			// Find one particular solution using extended Euclid
			_, x0, y0 := extendedEuclid(a1, b1)

			factor := z1 / d1
			xPart := x0 * factor
			yPart := y0 * factor

			// General solution:
			// x = xPart + (b1/d1)*t
			// y = yPart - (a1/d1)*t

			// Check if we can make y arbitrarily large:
			// bDivD := b1 / d1
			aDivD := a1 / d1

			// y = yPart - (a1/d1)*t
			// If a1/d1 != 0, we can choose t to make y as large as we want by choosing t negative or positive depending on sign.
			if aDivD != 0 {
				// We can make y as large as we like by choosing t accordingly -> no maximum
				return true, xPart, yPart, false
			} else {
				// aDivD = 0 means y is fixed (y = yPart).
				// In that case, that's the only y-value possible on this line (since bDivD*t affects only x).
				// That y is the maximum (and minimum, actually all solutions have the same y).
				return true, xPart, yPart, true
			}
		}

		// If not coincident, they are parallel and different -> no solution.
		return false, 0, 0, false
	}
}
