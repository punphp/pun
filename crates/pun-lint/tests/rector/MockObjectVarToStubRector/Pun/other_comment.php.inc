<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    // Rector reprints this comment with the changed doc below it; Pun leaves a doc after another comment alone.
    /** @var MockObject */
    private Stub $n;
}
