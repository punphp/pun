<?php

namespace CallableVisibility;

class Base
{
    private function hidden(): void {}
    private static function hiddenStatic(): void {}
    protected function guarded(): void {}
    public function exposed(): void {}

    public function inspect(): void
    {
        $private = [$this, 'hidden'];
        \PHPStan\dumpType($private());
        \PHPStan\dumpType(call_user_func($private));
        \PHPStan\dumpType(call_user_func_array($private, []));
        \PHPStan\dumpType(call_user_func([self::class, 'hiddenStatic']));
    }

    /** @return \Closure(): void */
    public function escape(): \Closure
    {
        return $this->hidden(...);
    }
}

class Child extends Base
{
    public function inspectProtected(): void
    {
        $protected = [$this, 'guarded'];
        \PHPStan\dumpType($protected());
        \PHPStan\dumpType(call_user_func($protected));
    }
}

function outside(Base $base, Child $child, bool $condition, mixed $mixed, object $object): void
{
    $private = [$base, 'hidden'];
    \PHPStan\dumpType($private());
    \PHPStan\dumpType(call_user_func($private));
    \PHPStan\dumpType(call_user_func_array(args: [], callback: $private));
    \PHPStan\dumpType(call_user_func(callback: [Base::class, 'hiddenStatic']));
    $privateString = Base::class . '::hiddenStatic';
    \PHPStan\dumpType($privateString());
    \PHPStan\dumpType(call_user_func($privateString));
    $protected = [$child, 'guarded'];
    \PHPStan\dumpType($protected());
    \PHPStan\dumpType(call_user_func_array($protected, []));
    $public = [$base, 'exposed'];
    \PHPStan\dumpType($public());
    \PHPStan\dumpType(call_user_func($public));
    $escaped = $base->escape();
    \PHPStan\dumpType($escaped());
    $union = $condition ? $private : $public;
    \PHPStan\dumpType($union());
    \PHPStan\dumpType(call_user_func($union));
    \PHPStan\dumpType($mixed());
    \PHPStan\dumpType($object());
    $integer = 17;
    \PHPStan\dumpType($integer());
    if (!is_callable($mixed)) {
        \PHPStan\dumpType($mixed());
    }
}
