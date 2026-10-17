<?php

// From PHPStan's Functions/data/intersection-types.php: Mago 1.57.0's analyser trips its own debug assertion
// (`TUnion::from_vec()` with an empty Vec) on the impossible `int&mixed`.
function doBaz(int&mixed $a): int&mixed
{
}
