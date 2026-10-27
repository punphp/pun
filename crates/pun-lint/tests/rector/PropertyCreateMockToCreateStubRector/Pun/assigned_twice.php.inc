<?php

namespace App\Tests\CreateStub;

use PHPUnit\Framework\TestCase;

final class AssignedTwiceTest extends TestCase
{
    private $assignedTwice;

    protected function setUp(): void
    {
        $this->assignedTwice = $this->createMock(\stdClass::class);
        $this->assignedTwice = $this->createMock(\DateTime::class);
    }
}
-----
<?php

namespace App\Tests\CreateStub;

use PHPUnit\Framework\TestCase;

final class AssignedTwiceTest extends TestCase
{
    private \PHPUnit\Framework\MockObject\Stub $assignedTwice;

    protected function setUp(): void
    {
        $this->assignedTwice = $this->createStub(\stdClass::class);
        $this->assignedTwice = $this->createStub(\DateTime::class);
    }
}
