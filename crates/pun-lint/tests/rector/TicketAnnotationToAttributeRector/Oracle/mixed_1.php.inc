<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

/**
 * Some text.
 *
 * @ticket 123
 * @ticket  JIRA-1  with spaces
 * @group foo
 */
#[\Foo]
final class SomeTest extends TestCase
{
    /** @ticket 9 */
    public function testA()
    {
    }

    /**
     * @ticket
     */
    #[Bar] public static function testB() {}

        /**
         * @TICKET 5
         */
        protected function testC() {}
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

/**
 * Some text.
 *
 * @group foo
 */
#[\Foo]
#[\PHPUnit\Framework\Attributes\Ticket('123')]
#[\PHPUnit\Framework\Attributes\Ticket('JIRA-1  with spaces')]
final class SomeTest extends TestCase
{
    #[\PHPUnit\Framework\Attributes\Ticket('9')]
    public function testA()
    {
    }

    #[Bar]
    #[\PHPUnit\Framework\Attributes\Ticket('')] public static function testB() {}

        /**
         * @TICKET 5
         */
        protected function testC() {}
}
