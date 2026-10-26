<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /**
     * @ticket (foo)
     * @ticket:5
     * @ticket a\\b
     * @ticket a\b\
     * @ticket it's
     * @ticket5
     * @group x(y)
     * @ticket after
     */
    public function testA()
    {
    }

    /**
     * @param int $a (
     * @ticket 6
     * @author me <me@example.com>
     */
    public function testB()
    {
    }

    /**
     * @group 'a(' "b)"
     * @ticket 7
     */
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
    /**
     * @ticket (foo)
     * @ticket5
     * @group x(y)
     */
    #[\PHPUnit\Framework\Attributes\Ticket(':5')]
    #[\PHPUnit\Framework\Attributes\Ticket('a\\\\b')]
    #[\PHPUnit\Framework\Attributes\Ticket('a\b\\')]
    #[\PHPUnit\Framework\Attributes\Ticket('it\'s')]
    #[\PHPUnit\Framework\Attributes\Ticket('after')]
    public function testA()
    {
    }

    /**
     * @param int $a (
     * @author me <me@example.com>
     */
    #[\PHPUnit\Framework\Attributes\Ticket('6')]
    public function testB()
    {
    }

    /**
     * @group 'a(' "b)"
     */
    #[\PHPUnit\Framework\Attributes\Ticket('7')]
    public function testC()
    {
    }
}
