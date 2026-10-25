<?php

namespace MethodOriginsReturns;

use function PHPStan\dumpType;
use function PHPStan\dumpPhpDocType;

class ParentReturns
{
    /** @return array{z: string, a: bool} */
    public function ordered(): array { return ['z' => 'x', 'a' => true]; }

    /** @return list<mixed> */
    public function explicit(): array { return []; }

    public function implicit(): array { return []; }

    /** @return array<mixed> */
    public static function staticExplicit(): array { return []; }

    /** @return array{z: string, a: bool} */
    public static function staticOrdered(): array { return ['z' => 'x', 'a' => true]; }

    /** @param static $other */
    public function staticReceiver(self $other): void { dumpType($other->ordered()); }
}

class ChildReturns extends ParentReturns
{
    public function examples(): void
    {
        dumpType($this->ordered(), $this->ExPlIcIt(), $this->implicit());
        dumpType(self::staticExplicit(), parent::staticOrdered(), static::staticOrdered());
    }
}

class OtherReturns
{
    /** @return array{a: bool, z: string} */
    public function ordered(): array { return ['a' => true, 'z' => 'x']; }

    /** @return array{a: bool, z: string} */
    public function array_slice(array $array): array { return ['a' => false, 'z' => 'method']; }
}

function examples(ParentReturns $object, ChildReturns $child, OtherReturns $other): void
{
    $copy = ($object->ordered());
    dumpPhpDocType($copy);
    dumpType($copy, $child->ordered(), $other->ordered());
    dumpType(ParentReturns::staticExplicit(), ChildReturns::staticOrdered());
    dumpType($object->explicit(), $object->implicit());
    dumpType($other->array_slice(['z' => 'input', 'a' => true]));
}
