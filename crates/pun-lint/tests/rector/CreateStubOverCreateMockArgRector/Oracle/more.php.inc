<?php

namespace Pun\Ns2;

abstract class MoreTest extends BaseTest
{
    abstract public function testAbstract();

    public function testMore(string $name)
    {
        $helper = new Helper;
        new Helper;
        new class {
        };
        $helper->$name($this->createMock(\stdClass::class));
        $helper->untyped($this->{'createMock'}(\stdClass::class));
        $helper->untyped($this->$name(\stdClass::class));
    }
}
