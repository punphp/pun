<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

/**
 * @ticket 1
 *
 * Some text.
 * @ticket 2
 *
 * @author me
 *
 * @ticket 3
 */
final class SomeTest extends TestCase
{
    /**
     * @ticket 4
     * @ticket 5
     */
    public function testA()
    {
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

/**
 * @author me
 *
 */
#[\PHPUnit\Framework\Attributes\Ticket('1
Some text.')]
#[\PHPUnit\Framework\Attributes\Ticket('2')]
#[\PHPUnit\Framework\Attributes\Ticket('3')]
final class SomeTest extends TestCase
{
    #[\PHPUnit\Framework\Attributes\Ticket('4')]
    #[\PHPUnit\Framework\Attributes\Ticket('5')]
    public function testA()
    {
    }
}
