<?php

declare(strict_types=1);

namespace NamedProducerConstructor;

/** @consistent-constructor */
class ParentClass
{
    public function __construct(string $initial) { echo $initial; }
}

class ChildClass extends ParentClass
{
    public function __construct(int $replacement) { echo $replacement; }
}

function invoke(ParentClass $object): void
{
    $object->__construct(initial: 'constructor');
}
