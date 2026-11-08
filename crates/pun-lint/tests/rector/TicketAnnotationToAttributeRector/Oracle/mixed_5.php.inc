<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /**
     * @ticket
     *    123
     * @author x
     *
     *   more author
     *
     * @ticket 7
     *
     * trailing
     */
    public function testA()
    {
    }

    /**
     * Text one.
     *
     * Text two.
     *
     * @ticket 1
     */
    public function testB()
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
     * @author x
     *
     *   more author
     */
    #[\PHPUnit\Framework\Attributes\Ticket('
   123')]
    #[\PHPUnit\Framework\Attributes\Ticket('7
trailing')]
    public function testA()
    {
    }

    /**
     * Text one.
     *
     * Text two.
     */
    #[\PHPUnit\Framework\Attributes\Ticket('1')]
    public function testB()
    {
    }
}
