<?php

namespace Tests\AbstractPrivate;

use PHPUnit\Framework\TestCase;

abstract class AbstractCaseTest extends TestCase
{
    public function testRun()
    {
        $this->helper(1);
    }

    abstract private function helper($value);
}
