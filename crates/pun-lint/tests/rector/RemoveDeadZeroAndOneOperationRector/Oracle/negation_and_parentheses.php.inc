<?php
function f(int $i, int $j) {
    $a = 0 - ($i + $j);
    $a = 0 - -$i;
    $a = 2 * ($i - 0);
    $a = 2 * $i + 0;
    $a = $j - ($i - 0);
}
-----
<?php
function f(int $i, int $j) {
    $a = -($i + $j);
    $a = -(-$i);
    $a = 2 * ($i);
    $a = 2 * $i;
    $a = $j - ($i);
}
