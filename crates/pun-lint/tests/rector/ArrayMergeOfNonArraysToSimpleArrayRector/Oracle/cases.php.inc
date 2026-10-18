<?php

function a($a, $b)
{
    $x = \ARRAY_MERGE(array(1, 2), [3 => $a, 'k' => ($b)], [&$a]);
    $x = array_merge([], []);
    $x = array_merge();
    $x = array_merge([1], $a);
    $x = array_merge(...[[1], [2]]);
    $x = array_merge(
        [
            // c
            1,
        ],
        [2],
    )[0];
    $x = Foo\array_merge([1], [2]);
    $x = array_merge([1], [...$a, ...$b]);
}
-----
<?php

function a($a, $b)
{
    $x = [1, 2, 3 => $a, 'k' => $b, $a];
    $x = array_merge([], []);
    $x = array_merge();
    $x = array_merge([1], $a);
    $x = [[1], [2]];
    $x = [1, 2][0];
    $x = Foo\array_merge([1], [2]);
    $x = [1, ...$a, ...$b];
}
