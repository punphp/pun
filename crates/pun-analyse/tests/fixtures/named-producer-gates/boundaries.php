<?php

declare(strict_types=1);

namespace NamedProducerBoundaries;

/** @consistent-constructor */
class ParentClass
{
    public function __construct(string $initial) { echo $initial; }
    public function run(string $old): void { echo $old; }
    public static function stat(string $old): void { echo $old; }
}

class ChildClass extends ParentClass
{
    public function __construct(string $replacement) { echo $replacement; }
    public function run(string $new): void { echo $new; }
    public static function stat(string $new): void { echo $new; }

    public function probe(): void
    {
        $this->missing();
    }
}

function negatives(ParentClass $object, ?ParentClass $nullable): void
{
    new ParentClass(initial: 'new does not collect');
    ParentClass::stat(old: 'static does not collect');
    $closure = $object->run(...);
    $object->missing();
    $nullable?->missing();
    $object->{123}();
}
