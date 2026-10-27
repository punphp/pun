<?php

namespace Rector\PHPUnit\Tests\CodeQuality\Rector\Class_\NarrowUnusedSetUpDefinedPropertyRector\Fixture;

use PHPUnit\Framework\TestCase;
use Rector\PHPUnit\Tests\CodeQuality\Rector\Class_\NarrowUnusedSetUpDefinedPropertyRector\Source\SomeTrait;

// The trait fetches `property`, not `other`: Rector reads it and makes `other` a local variable; Pun does not read a
// trait from another file and keeps it.
final class TraitInOtherFile extends TestCase
{
    use SomeTrait;

    private $other;

    protected function setUp(): void
    {
        $this->other = 1;
    }
}
