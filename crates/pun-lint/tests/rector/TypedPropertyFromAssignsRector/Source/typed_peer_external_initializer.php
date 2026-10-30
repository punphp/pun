<?php

namespace TypedConstructorExternal;

/** @template T */
class Initializer
{
    protected function initialize(): void
    {
        $this->value = 1;
    }
}

class ForeignInitializer
{
    public function initialize(): void
    {
        $this->value = 1;
    }
}
