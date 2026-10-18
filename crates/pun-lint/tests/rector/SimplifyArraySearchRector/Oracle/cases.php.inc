<?php

function a($a, $b)
{
    $x = \ARRAY_SEARCH($a, $b) !== FALSE;
    $x = array_search(
        $a,
        $b,
    ) === false;
    $x = array_search($a, $b) === false && $c;
    $x = array_search(...$a) !== false;
    $x = array_search(needle: $a, haystack: $b) !== false;
    $x = Foo\array_search($a, $b) !== false;
    $x = array_search($a, $b) == false;
    $x = array_search($a, $b) === null;
    $x = false === array_search($a, $b) . 'x';
}
-----
<?php

function a($a, $b)
{
    $x = in_array($a, $b);
    $x = !in_array($a, $b);
    $x = !in_array($a, $b) && $c;
    $x = in_array(...$a);
    $x = in_array(needle: $a, haystack: $b);
    $x = Foo\array_search($a, $b) !== false;
    $x = array_search($a, $b) == false;
    $x = array_search($a, $b) === null;
    $x = false === array_search($a, $b) . 'x';
}
