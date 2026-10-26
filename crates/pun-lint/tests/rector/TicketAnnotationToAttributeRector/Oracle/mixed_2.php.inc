<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /**
     * @ticket first
     *   continued line
     * text after
     *
     * @ticket(foo)
     */
    public function testA() {}

    /** @ticket it's \ here */ public function testB() {}

    /**
     * Text.
     * @ticket 7
     */
    #[A]
    #[B] public function testC() {}

    /** @ticket 8 */
    #[A] #[B]
    public function testD() {}
}

/**
 * @ticket 1
 */
abstract class AbstractTest extends TestCase {}

/** @ticket 2 */
class NotTest {}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /**
     * @ticket(foo)
     */
    #[\PHPUnit\Framework\Attributes\Ticket('first
  continued line
text after')]
    public function testA() {}

    #[\PHPUnit\Framework\Attributes\Ticket('it\'s \ here')]
    public function testB() {}

    /**
     * Text.
     */
    #[A]
    #[B]
    #[\PHPUnit\Framework\Attributes\Ticket('7')] public function testC() {}

    #[A] #[B]
    #[\PHPUnit\Framework\Attributes\Ticket('8')]
    public function testD() {}
}

#[\PHPUnit\Framework\Attributes\Ticket('1')]
abstract class AbstractTest extends TestCase {}

/** @ticket 2 */
class NotTest {}
