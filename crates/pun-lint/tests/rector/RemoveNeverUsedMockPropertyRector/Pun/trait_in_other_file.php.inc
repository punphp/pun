<?php

namespace Rector\PHPUnit\Tests\CodeQuality\Rector\Class_\RemoveNeverUsedMockPropertyRector\Fixture;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;
use Rector\PHPUnit\Tests\CodeQuality\Rector\Class_\RemoveNeverUsedMockPropertyRector\Source\UsesMockPropertyTrait;

// The trait fetches `mockProperty`, not `otherProperty`: Rector reads it and removes `otherProperty`; Pun does not
// read a trait from another file and keeps it.
final class TraitInOtherFile extends TestCase
{
    use UsesMockPropertyTrait;

    private MockObject $otherProperty;

    protected function setUp(): void
    {
        $this->otherProperty = $this->createMock(\stdClass::class);
    }
}
