<?php

namespace MethodOriginsControls;

use function PHPStan\dumpType;

class Api
{
    public function implicit(...$args): array { return []; }

    public static function staticImplicit(): array { return []; }

    /** @param-out array{a: bool, z: string} $value */
    public function replace(array &$value): void { $value = ['a' => true, 'z' => 'x']; }

    /** @return array{z: string, a: bool} */
    public function ordered(): array { return ['z' => 'x', 'a' => true]; }

    /** @return array{z: string, a: bool} */
    public static function staticOrdered(): array { return ['z' => 'x', 'a' => true]; }
}

class OtherApi
{
    public function implicit(): array { return []; }
}

/** @param array{z: string, a: bool} $value */
function controls(Api $api, ?Api $nullable, Api|OtherApi $union, array $value, array $args): void
{
    $method = 'implicit';
    dumpType($api->$method());
    $class = Api::class;
    dumpType($class::staticImplicit());
    dumpType($union->implicit());
    dumpType($api->implicit(...$args));
    dumpType($nullable?->implicit());
    dumpType((new \DateTimeImmutable())->getTimestamp());
    $api->replace($value);
    dumpType($value);
    $copy = $api->ordered();
    unset($copy['z']);
    $copy['z'] = 'changed';
    dumpType($copy);
}
