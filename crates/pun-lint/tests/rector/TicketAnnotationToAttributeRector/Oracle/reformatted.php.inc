<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /**
       * Text.

  * @ticket 5
   * @author me
        */
    public function testA()
    {
    }

    /**
     Text.
     @ticket 6
     @author x
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
     * Text.
     * @author me
     */
    #[\PHPUnit\Framework\Attributes\Ticket('5')]
    public function testA()
    {
    }

    /**
    Text.
    @author x
    */
    #[\PHPUnit\Framework\Attributes\Ticket('6')]
    public function testB()
    {
    }
}
