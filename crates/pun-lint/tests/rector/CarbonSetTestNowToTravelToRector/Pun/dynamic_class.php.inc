<?php

namespace App;

use Illuminate\Foundation\Testing\TestCase;

class DynamicTest extends TestCase
{
    public function test($class)
    {
        $class::setTestNow();
    }
}
