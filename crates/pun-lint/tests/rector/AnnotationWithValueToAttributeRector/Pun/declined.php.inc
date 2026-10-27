<?php

namespace App\Tests\Declined;

use PHPUnit\Framework\TestCase;

/**
 * @_internal
 * @backupGlobals enabled
 */
final class EscapedClassTest extends TestCase
{
}

final class EscapedMethodTest extends TestCase
{
    /**
     * @\Some\Annotation
     * @backupGlobals enabled
     */
    public function testIt(): void
    {
    }
}

final class TabbedMethodTest extends TestCase
{
	/**
	 * @backupGlobals enabled
	 */
	public function testIt(): void
	{
	}
}

	/**
	 * @uses \stdClass
	 */
	final class TabbedClassTest extends TestCase
	{
	}

final class AnonymousTest extends TestCase
{
    public function testAnonymous(): void
    {
        new #[\Some\Attribute] class extends TestCase {
            /**
             * @uses \stdClass
             */
            public function testInner(): void
            {
            }
        };
    }
}
