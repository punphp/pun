<?php

namespace App\Tests\Unnamed7;

use PHPUnit\Framework\TestCase;

final class UnnamedTest extends TestCase
{
    public function testIt($a, $b): void
    {
        $this->assertTrue($a instanceof $b[0]);
    }
}
-----
<?php

declare(strict_types=1);

namespace App\Tests\Unnamed7;

use PHPUnit\Framework\TestCase;

final class UnnamedTest extends TestCase
{
    public function testIt($a, $b): void
    {
        $this->assertTrue($a instanceof $b[0]);
    }
}
