<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /** @ticket 5 @author x */ public function testA() {}

    /** Text. */ /** @ticket 6
     * @group g */ public function testB() {}
}
