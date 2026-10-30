<?php

namespace ReturnContractUnion;

final class Item
{
}

function item(): Item
{
    return new Item();
}

function number(): int
{
    return 42;
}

function anyObject(): object
{
    return new \stdClass();
}

function classFirst(bool $pick)
{
    if ($pick) {
        return item();
    }

    return number();
}

function classLast(bool $pick)
{
    if ($pick) {
        return number();
    }

    return item();
}

function redundantObject(bool $pick)
{
    if ($pick) {
        return item();
    }

    return anyObject();
}
-----
<?php

namespace ReturnContractUnion;

final class Item
{
}

function item(): Item
{
    return new Item();
}

function number(): int
{
    return 42;
}

function anyObject(): object
{
    return new \stdClass();
}

function classFirst(bool $pick): \ReturnContractUnion\Item|int
{
    if ($pick) {
        return item();
    }

    return number();
}

function classLast(bool $pick): int|\ReturnContractUnion\Item
{
    if ($pick) {
        return number();
    }

    return item();
}

function redundantObject(bool $pick)
{
    if ($pick) {
        return item();
    }

    return anyObject();
}
