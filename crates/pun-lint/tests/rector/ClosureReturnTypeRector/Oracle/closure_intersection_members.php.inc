<?php

namespace App;

$callableMember = function (\Countable $x) {
    if (is_callable($x)) {
        return $x;
    }

    throw new \Exception();
};

$iterableMember = function (\Countable $x) {
    if (is_iterable($x)) {
        return $x;
    }

    throw new \Exception();
};

/**
 * @template T of object
 * @param T $x
 */
function templated(object $x)
{
    return function () use ($x) {
        if ($x instanceof \Countable) {
            return $x;
        }

        throw new \Exception();
    };
}
-----
<?php

namespace App;

$callableMember = function (\Countable $x) {
    if (is_callable($x)) {
        return $x;
    }

    throw new \Exception();
};

$iterableMember = function (\Countable $x) {
    if (is_iterable($x)) {
        return $x;
    }

    throw new \Exception();
};

/**
 * @template T of object
 * @param T $x
 */
function templated(object $x)
{
    return function () use ($x): \Countable {
        if ($x instanceof \Countable) {
            return $x;
        }

        throw new \Exception();
    };
}
