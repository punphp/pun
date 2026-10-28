<?php

declare(strict_types=1);

namespace NamedProducerGates;

class ParentMethod
{
    public function run(string $old): void { echo $old; }
}

class RenamedMethod extends ParentMethod
{
    public function run(string $new): void { echo $new; }
}

class IncompatibleMethod extends ParentMethod
{
    public function run(int $changed): void { echo $changed; }
}

/** @consistent-constructor */
class ParentConstructor
{
    public function __construct(string $initial) { echo $initial; }
}

class RenamedConstructor extends ParentConstructor
{
    public function __construct(string $replacement) { echo $replacement; }
}

function invoke(ParentMethod $method, ParentConstructor $constructor): void
{
    $method->run(old: 'method');
    $constructor->__construct(initial: 'constructor');
    new ParentConstructor(initial: 'new does not collect');
    $closure = $method->run(...);
}
