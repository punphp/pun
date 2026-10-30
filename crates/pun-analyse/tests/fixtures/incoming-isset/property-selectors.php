<?php

namespace IssetPropertySelectors;

class Base
{
    public int $value = 1;
}

class Child extends Base {}

function selectors(Base $object): void
{
    $name = 'value';
    echo isset($object->{$name});
    echo isset($object->{'value'});
}

function excludingSubtype(Base $object): bool
{
    if ($object instanceof Child) {
        return false;
    }
    return isset($object->value);
}

function methodGuard(Base $object): bool
{
    if (method_exists($object, 'extra')) {
        return isset($object->value);
    }
    return false;
}
