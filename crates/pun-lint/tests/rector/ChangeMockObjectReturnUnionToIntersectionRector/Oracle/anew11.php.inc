<?php

use PHPUnit\Framework\TestCase;

final class RefTest extends TestCase
{
    /**
     * Doc.
     */
    #[\Override]
    final protected function &setUp(int $a = 1, string ...$b): void
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
