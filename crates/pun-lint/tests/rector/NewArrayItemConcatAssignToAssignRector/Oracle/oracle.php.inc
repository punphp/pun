<?php

function newArrayItemOracle($values, $name)
{
    $values[] .= $name . 'x';
    $values[] += 1;
    $values .= $name;
    $values['a'][] .= $name;
}
-----
<?php

function newArrayItemOracle($values, $name)
{
    $values[] = $name . 'x';
    $values[] += 1;
    $values .= $name;
    $values['a'][] = $name;
}
