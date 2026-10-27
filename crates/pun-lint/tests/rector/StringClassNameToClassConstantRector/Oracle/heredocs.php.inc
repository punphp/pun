<?php

namespace N;

function f()
{
    $a = <<<X
        Random\Randomizer
        X;
    $c = <<<X
Random\Randomizer

X;
    $d = <<<X
    Random\Randomizer
    {$x}
    X;
    $e = <<<X
Random\Randomizer
X;
    $f = <<<X
      Random\Engine\Mt19937
    X;
    $h = <<<X
	Random\Randomizer
	X;
    $i = <<<X
  Random\Randomizer
    Random\Randomizer
  X;
}
-----
<?php

namespace N;

function f()
{
    $a = \Random\Randomizer::class;
    $c = <<<X
Random\Randomizer

X;
    $d = <<<X
    Random\Randomizer
    {$x}
    X;
    $e = \Random\Randomizer::class;
    $f = <<<X
      Random\Engine\Mt19937
    X;
    $h = \Random\Randomizer::class;
    $i = <<<X
  Random\Randomizer
    Random\Randomizer
  X;
}
