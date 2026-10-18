<?php

abstract class B
{
    final protected static function &b(
        // the value
        $x,
        /** more */ int ...$rest
    ) {
        // keep
        if ($x !== null) {
            /* inner */
            return $x;
        }
        return null;
    }
}

function c($x): ?string
{
    if (null !== $x) {
        return $x;
    }

    return null;
}

$g = static fn () => 1;
$h = #[Pure] static function &() use ($a, &$b): mixed {
    if ($a !== null) {
        return $a;
    }
    return null;
};
-----
<?php

abstract class B
{
    final protected static function &b(
        // the value
        $x,
        /** more */
        int ...$rest
    )
    {
        /* inner */
        return $x;
    }
}

function c($x): ?string
{
    return $x;
}

$g = static fn () => 1;
$h = #[Pure] static function &() use ($a, &$b): mixed {
    return $a;
};
