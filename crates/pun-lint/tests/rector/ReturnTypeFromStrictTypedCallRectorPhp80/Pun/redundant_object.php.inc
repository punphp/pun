<?php

namespace ReturnContractUnionPun;

final class Item
{
}

function item(): Item
{
    return new Item();
}

function anyObject(): object
{
    return new \stdClass();
}

function redundantObject(bool $pick)
{
    if ($pick) {
        return item();
    }

    return anyObject();
}
