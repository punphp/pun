<?php

namespace Probe;

/**
 * @param  array{a: 1, b: 2}  $x
 */
function two($x)
{
    return $x['a'];
}

/**
 * @param  array{a: 1, b: 1}  $x
 */
function same($x)
{
    return $x['a'];
}

/**
 * @param  array{a: 'x'}  $x
 */
function one($x)
{
    return $x['a'];
}

/**
 * @param  array{a: true, b: false}  $x
 */
function bools($x)
{
    return $x['a'];
}

/**
 * @param  array{a: 'x', b: 'y'}  $x
 */
function strings($x)
{
    return $x['a'];
}

function closures()
{
    return array_map(function ($row) {
        return $row['id'];
    }, [['id' => 1], ['id' => 2]]);
}
-----
<?php

namespace Probe;

/**
 * @param  array{a: 1, b: 2}  $x
 */
function two($x)
{
    return $x['a'];
}

/**
 * @param  array{a: 1, b: 1}  $x
 */
function same(array $x)
{
    return $x['a'];
}

/**
 * @param  array{a: 'x'}  $x
 */
function one(array $x)
{
    return $x['a'];
}

/**
 * @param  array{a: true, b: false}  $x
 */
function bools(array $x)
{
    return $x['a'];
}

/**
 * @param  array{a: 'x', b: 'y'}  $x
 */
function strings($x)
{
    return $x['a'];
}

function closures()
{
    return array_map(function ($row) {
        return $row['id'];
    }, [['id' => 1], ['id' => 2]]);
}
