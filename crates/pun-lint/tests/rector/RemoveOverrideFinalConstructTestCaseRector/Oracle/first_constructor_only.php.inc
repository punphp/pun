<?php

namespace App;

use PHPUnit\Framework\TestCase;

final class HTest extends TestCase
{
    public function test(): void
    {
    }

    /** doc */
    public function __CONSTRUCT()
    {
        parent::__construct('x');
    }

    public function testTwo(): void
    {
    }
}

final class NotATest
{
    public function __construct()
    {
    }
}

$x = new class extends TestCase {
    public function __construct()
    {
    }
};
-----
<?php

namespace App;

use PHPUnit\Framework\TestCase;

final class HTest extends TestCase
{
    public function test(): void
    {
    }

    public function testTwo(): void
    {
    }
}

final class NotATest
{
    public function __construct()
    {
    }
}

$x = new class extends TestCase
{
};
