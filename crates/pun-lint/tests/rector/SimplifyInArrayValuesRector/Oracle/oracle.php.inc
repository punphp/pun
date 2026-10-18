<?php

function inArrayValuesOracle($needle, $haystack, $f)
{
    $r = \in_array($needle, \array_values($haystack));
    $r = in_array($needle, (array_values(($haystack))));
    $r = in_array($needle, $haystack);
    $r = in_array($needle);
    $r = in_array($needle, array_keys($haystack));
    $r = $f($needle, array_values($haystack));
    $r = in_array($needle, array_values(array_filter($haystack)), true);
}
-----
<?php

function inArrayValuesOracle($needle, $haystack, $f)
{
    $r = \in_array($needle, $haystack);
    $r = in_array($needle, ($haystack));
    $r = in_array($needle, $haystack);
    $r = in_array($needle);
    $r = in_array($needle, array_keys($haystack));
    $r = $f($needle, array_values($haystack));
    $r = in_array($needle, array_filter($haystack), true);
}
