<?php

namespace App\Tests\Cover13;

use PHPUnit\Framework\TestCase;

use App\Other\Thing as Depends;

final class Cover13Test extends TestCase
{
    public function testA(): int
    {
        return 1;
    }

    /**
     * @depends testA
     */
    public function testB($a): void
    {
    }
}
-----
<?php

declare(strict_types=1);

namespace App\Tests\Cover13;

use PHPUnit\Framework\TestCase;

use App\Other\Thing as Depends;

final class Cover13Test extends TestCase
{
    public function testA(): int
    {
        return 1;
    }

    /**
     * @depends testA
     */
    public function testB($a): void
    {
    }
}
