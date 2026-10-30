<?php

namespace App;

trait WritesValue
{
    public function reset(): void
    {
        $this->value = 0;
    }
}

class Base
{
    protected $inherited;
}

final class Child extends Base
{
    use WritesValue;

    private $value;
    protected $own;
    protected $inherited;

    public function __construct()
    {
        $this->value = 1;
        $this->own = 'x';
        $this->inherited = 'y';
    }
}

class NotFinal
{
    protected $own;

    public function __construct()
    {
        $this->own = 'x';
    }
}

class WithMetadata
{
    private $mapped;

    public function __construct()
    {
        $this->mapped = 1;
    }

    public static function loadMetadata($metadata): void
    {
    }
}
