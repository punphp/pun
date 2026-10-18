<?php

namespace AttributeNoDiscard;

#[\NoDiscard]
function nativeVoid(): VOID {}

#[\nodiscard]
function never(): never { throw new \Exception(); }

/** @return void */
#[\NoDiscard]
function docVoid() {}

/** @return never */
#[\NoDiscard]
function docNever() { throw new \Exception(); }

/** @return int */
#[\NoDiscard]
function docInt() { return 1; }

#[\NoDiscard]
function nativeInt(): ?int { return 1; }

#[NoDiscard]
function notTheBuiltin(): void {}

$f = #[\NoDiscard] function (): Void {};
$g = #[\NoDiscard] static fn (): never => throw new \Exception();
$h = #[\NoDiscard] function () {};

class Magic
{
    #[\NoDiscard] public function __set($a, $b) {}
    #[\NoDiscard] public function __unserialize(array $a) {}
    #[\NoDiscard] public function __toString(): string { return ''; }
    /** @return void */
    #[\NoDiscard] public function doc() {}
    #[\NoDiscard] public function plain() {}
}

trait T
{
    #[\NoDiscard]
    public function fromTrait(): void {}
}

class UsesTrait
{
    use T;
}
