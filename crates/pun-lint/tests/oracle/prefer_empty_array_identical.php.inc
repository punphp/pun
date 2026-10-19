<?php

function after(array $guards)
{
    if (empty($guards)) {
        $guards = [null];
    }

    return $guards;
}

function before(array $items)
{
    $items = foo();

    return empty($items);
}

function in_loop(array $items)
{
    while (true) {
        if (! empty($items)) {
            return $items;
        }

        $items = foo();
    }
}

function same_expression(array $items)
{
    return empty($items) || ($items = foo());
}

function closure_after(array $items)
{
    return function (array $values) {
        if (empty($values)) {
            return null;
        }

        $values = foo();

        return $values;
    };
}
-----
<?php

function after(array $guards)
{
    if ($guards === []) {
        $guards = [null];
    }

    return $guards;
}

function before(array $items)
{
    $items = foo();

    return empty($items);
}

function in_loop(array $items)
{
    while (true) {
        if (! empty($items)) {
            return $items;
        }

        $items = foo();
    }
}

function same_expression(array $items)
{
    return $items === [] || ($items = foo());
}

function closure_after(array $items)
{
    return function (array $values) {
        if ($values === []) {
            return null;
        }

        $values = foo();

        return $values;
    };
}
