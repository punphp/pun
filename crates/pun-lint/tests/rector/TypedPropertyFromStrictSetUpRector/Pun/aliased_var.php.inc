<?php

namespace App;

use DateTime as Clock;
use PHPUnit\Framework\MockObject\MockObject as Mock;
use PHPUnit\Framework\TestCase;

final class AliasedVarTest extends TestCase
{
    /** @var Clock */
    private $clock;
    /** @var Mock */
    private $mock;
    private $nullDefault = null;

    protected function setUp(): void
    {
        $this->clock = $this->createMock(\DateTime::class);
        $this->mock = $this->createMock(\DateTime::class);
        $this->nullDefault = 10;
    }
}

?>
-----
<?php

namespace App;

use DateTime as Clock;
use PHPUnit\Framework\MockObject\MockObject as Mock;
use PHPUnit\Framework\TestCase;

final class AliasedVarTest extends TestCase
{
    /** @var (\PHPUnit\Framework\MockObject\MockObject & \DateTime) */
    private \PHPUnit\Framework\MockObject\MockObject $clock;
    /** @var Mock */
    private \PHPUnit\Framework\MockObject\MockObject $mock;
    private $nullDefault = null;

    protected function setUp(): void
    {
        $this->clock = $this->createMock(\DateTime::class);
        $this->mock = $this->createMock(\DateTime::class);
        $this->nullDefault = 10;
    }
}

?>
