<?php
namespace UnsetWritableGenericScope;
/**
 * @template T of string
 * @template U of int
 * @property-read T $p
 */
class GenericScope
{
    public mixed $p;
    public function write(): void
    {
        $this->p = 1;
        $this->p = 'allowed';
    }
}
/** @param GenericScope<non-empty-string, positive-int> $o */
function write(GenericScope $o): void
{
    $o->p = 1;
    $o->p = '';
}

function defaultBounds(GenericScope $o): void
{
    $o->p = 1;
    $o->p = 'allowed';
}
