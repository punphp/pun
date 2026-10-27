<?php

namespace App\Tests\Ctor;

use PHPUnit\Framework\TestCase;

abstract class AbstractConstructorTest extends TestCase
{
    abstract public function __construct();
}

abstract class AbstractSetUpTest extends TestCase
{
    public function __construct()
    {
        $this->value = 1;
    }

    abstract protected function setUp(): void;
}
