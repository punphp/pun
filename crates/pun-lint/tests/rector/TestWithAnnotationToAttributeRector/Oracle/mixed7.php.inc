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

namespace App\Tests\Mixed7;

use PHPUnit\Framework\TestCase;

final class MixedIndentTest extends TestCase
{
	private int $tabbed = 1;

    #[\PHPUnit\Framework\Attributes\TestWith([1])]
    public function testIt(int $a): void
    {
    }
}
