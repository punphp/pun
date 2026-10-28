<?php

/**
 * @param int|array $value
 */
function intOrArray($value)
{
    return $value[0];
}

/**
 * @param list{int, string} $pair
 */
function pair($pair)
{
    return $pair[0];
}

/**
 * @param list{int, int} $pair
 */
function samePair($pair)
{
    return $pair[0];
}
-----
<?php

/**
 * @param int|array $value
 */
function intOrArray($value)
{
    return $value[0];
}

/**
 * @param list{int, string} $pair
 */
function pair($pair)
{
    return $pair[0];
}

/**
 * @param list{int, int} $pair
 */
function samePair(array $pair)
{
    return $pair[0];
}
