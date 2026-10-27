<?php

namespace App\Tests\Cover4;

use PHPUnit\Framework\TestCase;

$this->assertTrue(!$a);
$this->assertSame(null, $a);

function helper($a)
{
    $this->assertTrue(!$a);
}

final class Cover4Test extends TestCase
{
    public function testConstants($a, $b): void
    {
        $this->assertSame(\true, $a);
        $this->assertSame(\NULL, $a);
        $this->assertSame(FALSE, $a);
        $this->assertNotSame(\false, $a);
        $this->assertIsType(1, $a);
        $this->assertContainsOnly(1, $a);
        $this->createMock(static::class);
        $this->createMock($a::class);
        $this->assertTrue(['a' => 1] == $b);
        $this->assertTrue($this->keyed() == $b);
        $this->assertTrue(
            in_array($a, $b),
            'message'
        );
    }

    public function testNames($a, $b, $f): void
    {
        $this->assertFalse($a->{$b});
        $this->assertFalse($a?->strpos);
        $this->assertFalse($a?->{$b});
        $this->assertFalse(self::$strpos);
        $this->assertFalse(self::${'x'});
        $this->assertFalse(STRPOS);
        $this->assertFalse($f('x'));
        $this->assertFalse($strpos);
    }

    /**
     * @return array<string, int>
     */
    private function keyed(): array
    {
        return [];
    }
}
