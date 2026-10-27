<?php

namespace App\Tests\IntersectionParam;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class MultilineParamTest extends TestCase
{
    public function testCalls(): void
    {
        $this->multiline($this->createMock(\stdClass::class));
    }

    /**
     * @param MockObject
     *     $first
     */
    private function multiline(MockObject $first): void
    {
    }
}

final class TabbedParamTest extends TestCase
{
	public function testCalls(): void
	{
		$this->helper($this->createMock(\stdClass::class));
	}

	private function helper(MockObject $first): void
	{
	}
}
