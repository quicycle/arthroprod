.: Arthroprod :.
================
_Compute products under Absolute Relativity._

Copyright (C) 2016-2017 Innes D. Anderson-Morrison All rights reserved.

![Cayley Table for the Williamson Algebra](cayley.png)

This is a Rust re-implementation of the Python framework [arpy](https://github.com/sminez/arpy).
It is intended to provide better confidence in the correctness of the algorithms
and also provide a speed up in computation for when we need to iterate on large
calculations.

(arpy is a module for performing calculations within the theory of Absolute Relativity
as devised by [Dr J.G.Williamson](http://www.gla.ac.uk/schools/engineering/staff/johnwilliamson/).)

The primary project can still be found [here](https://github.com/sminez/arpy)
and this may become a Python extension module in the future.


TODO
----
- [ ] Prodable as a trait? (Convert argument to a multivector and then ^)
- [x] MultiVectors
- [ ] Differentials
- [ ] Parsing of calculation files
