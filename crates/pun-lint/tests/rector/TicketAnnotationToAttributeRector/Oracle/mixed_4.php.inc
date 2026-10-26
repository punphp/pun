<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /**
     * @ticket a
     *
     *
     * b
     *   c
     *
     */
    public function testA()
    {
    }

    /**
     * @ticket f
     *
     * @ticket g */
    public function testC()
    {
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    #[\PHPUnit\Framework\Attributes\Ticket('a
b
  c')]
    public function testA()
    {
    }

    #[\PHPUnit\Framework\Attributes\Ticket('f')]
    #[\PHPUnit\Framework\Attributes\Ticket('g')]
    public function testC()
    {
    }
}
