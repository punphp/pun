<?php

namespace N;

$a = 'Random\Randomizer';
$b = 'Random\Engine\Mt19937';
$c = 'Random\Engine\Secure';
$d = 'Dom\Document';
$e = 'Dom\HTMLDocument';
$g = 'Random\Engine';
$h = 'Dom\Element';
$i = 'Dom\Node';
$j = 'Random\Engine\Xoshiro256StarStar';
$k = 'Dom\XMLDocument';
$l = 'Dom\Text';
$n = $fn('Random\Engine');
-----
<?php

namespace N;

$a = 'Random\Randomizer';
$b = 'Random\Engine\Mt19937';
$c = 'Random\Engine\Secure';
$d = 'Dom\Document';
$e = \Dom\HTMLDocument::class;
$g = \Random\Engine::class;
$h = 'Dom\Element';
$i = \Dom\Node::class;
$j = \Random\Engine\Xoshiro256StarStar::class;
$k = \Dom\XMLDocument::class;
$l = 'Dom\Text';
$n = $fn(\Random\Engine::class);
