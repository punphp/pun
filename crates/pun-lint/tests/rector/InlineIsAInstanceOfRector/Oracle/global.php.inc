<?php

interface Shape {}

function check(object $o, ?Shape $maybe, string $class)
{
    if (is_a($o, Shape::class)) {
        return is_a($maybe ?? $o, Shape::class) && is_a($o, $class);
    }

    return is_a($o, 'Shape');
}
-----
<?php

interface Shape {}

function check(object $o, ?Shape $maybe, string $class)
{
    if ($o instanceof \Shape) {
        return ($maybe ?? $o) instanceof \Shape && $o instanceof $class;
    }

    return is_a($o, 'Shape');
}
