<?php

namespace App\Tests\Mixed7;

use PHPUnit\Framework\TestCase;

final class MixedIndentTest extends TestCase
{
	private int $tabbed = 1;

    /**
     * @testWith [1]
     */
    public function testIt(int $a): void
    {
    }
}
-----
<?php

declare(strict_types=1);

namespace App\Tests\Mixed7;

use PHPUnit\Framework\TestCase;

final class MixedIndentTest extends TestCase
{
	private int $tabbed = 1;

    /**
     * @testWith [1]
     */
    public function testIt(int $a): void
    {
    }
}
