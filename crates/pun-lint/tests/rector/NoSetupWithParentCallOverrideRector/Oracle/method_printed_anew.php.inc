<?php

use PHPUnit\Framework\TestCase;

final class BTest extends TestCase
{
    /**
     * Doc.
     */
    #[\Override]
    final protected function setUp(int $a = 1, string ...$b): void
    {
        // first
        parent::setUp();

        $some = [
            1,
        ]; // trailing

        // end
    }

    #[\Override] protected function tearDown(
        // c
        $x,
    ) {
        parent::tearDown();
    }
}
-----
<?php

use PHPUnit\Framework\TestCase;

final class BTest extends TestCase
{
    /**
     * Doc.
     */
    final protected function setUp(int $a = 1, string ...$b): void
    {
        // first
        parent::setUp();
        $some = [
            1,
        ];
        // trailing
        // end
    }

    protected function tearDown(
        // c
        $x
    )
    {
        parent::tearDown();
    }
}
