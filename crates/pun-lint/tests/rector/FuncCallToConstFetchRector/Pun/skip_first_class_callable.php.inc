<?php

// Rector writes `M_PI` for `\pi(...)`: a float where the code had a callable. Pun leaves a first-class callable
// alone.

$f = \pi(...);
