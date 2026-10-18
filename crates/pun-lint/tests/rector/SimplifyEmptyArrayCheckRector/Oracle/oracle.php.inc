<?php

function emptyArrayCheckOracle($values, $other, $f)
{
    $r = \is_array($values) && empty(($values));
    $r = IS_ARRAY($values) && EMPTY($values);
    $r = is_array($values) and empty($values);
    $r = is_array($values) && empty($other);
    $r = empty($values) && is_array($other);
    $r = is_array() && empty($values);
    $r = is_array($values->a) && empty($values->a);
    $r = $f($values) && empty($values);
    $r = !(is_array($values) && empty($values));
    $r = is_array($values) && empty($values) && $other;
}
-----
<?php

function emptyArrayCheckOracle($values, $other, $f)
{
    $r = $values === [];
    $r = $values === [];
    $r = is_array($values) and empty($values);
    $r = is_array($values) && empty($other);
    $r = empty($values) && is_array($other);
    $r = is_array() && empty($values);
    $r = is_array($values->a) && empty($values->a);
    $r = $f($values) && empty($values);
    $r = !($values === []);
    $r = $values === [] && $other;
}
