<?php

namespace App\Tests\Decl11;

use PHPUnit\Framework\TestCase;

trait CoversTrait
{
    /**
     * @\Foo
     * @covers \Foo
     */
    public function testUnread(): void
    {
    }
}

trait TabbedTrait
{
	/**
	 * @covers \Foo
	 */
	public function testTabbed(): void
	{
	}
}

final class AnonymousTest extends TestCase
{
    public function testIt(): void
    {
        $anonymous = new readonly class extends TestCase {
            /**
             * @covers \Foo
             */
            public function testInner(): void
            {
            }
        };
    }
}
