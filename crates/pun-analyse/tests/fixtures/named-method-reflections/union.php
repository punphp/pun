<?php

namespace AuditCombined;

class User
{
    public function offsetSet(mixed ...$key): void
    {
    }
}

class Renamed extends User
{
    public function offsetSet(mixed ...$renamed): void
    {
    }
}

class RenamedArray extends \ArrayObject
{
    public function offsetSet(mixed $renamed, mixed $value): void
    {
    }
}

function userFirst(User|\ArrayObject $object): void
{
    $object->offsetSet(extra: 1);
    $object->offsetSet(key: 1);
}

function builtinFirst(\ArrayObject|User $object): void
{
    $object->offsetSet(extra: 1);
    $object->offsetSet(key: 1);
}
