<?php

namespace App\Tests\Cr11;

use PHPUnit\Framework\TestCase;

final class Cr11Test extends TestCase
{
    /**
     * ab
     * @dataProvider provide
     */
    public function testIt($a): void
    {
    }

    public static function provide(): array
    {
        return [[1]];
    }
}
-----
<?php

declare(strict_types=1);

namespace App\Tests\Cr11;

use PHPUnit\Framework\TestCase;

final class Cr11Test extends TestCase
{
    /**
     * ab
     * @dataProvider provide
     */
    public function testIt($a): void
    {
    }

    public static function provide(): array
    {
        return [[1]];
    }
}
