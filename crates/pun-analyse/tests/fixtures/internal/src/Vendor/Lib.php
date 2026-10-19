<?php

namespace Vendor\Pkg;

class Lib
{
    /** @internal */
    public function hidden(): void
    {
    }

    /** @internal */
    public static function hiddenStatic(): void
    {
    }

    public function open(): void
    {
        $this->hidden();
    }
}

/** @internal */
class Secret
{
    public function run(): void
    {
    }

    public static function make(): static
    {
        return new static();
    }
}

/** @internal */
interface SecretContract
{
    public function contract(): void;
}

class Child extends Secret
{
}

class Implementer implements SecretContract
{
    public function contract(): void
    {
    }
}
