<?php

namespace LintExportAudit;

use PHPUnit\Framework\TestCase;

final class NeverPublicTest extends TestCase
{
    public function testThrowExpression(): void
    {
        $this->assertEquals(1, throw new \RuntimeException('expected'));
    }

    public function testNeverCall(): void
    {
        $this->assertEquals(1, $this->stop());
    }

    public function testFiniteControl(): void
    {
        $this->assertEquals(1, 1);
    }

    public function testNullControl(): void
    {
        $this->assertEquals(1, null);
    }

    private function stop(): never
    {
        throw new \RuntimeException('expected');
    }
}
-----
<?php

namespace LintExportAudit;

use PHPUnit\Framework\TestCase;

final class NeverPublicTest extends TestCase
{
    public function testThrowExpression(): void
    {
        $this->assertEquals(1, throw new \RuntimeException('expected'));
    }

    public function testNeverCall(): void
    {
        $this->assertEquals(1, $this->stop());
    }

    public function testFiniteControl(): void
    {
        $this->assertSame(1, 1);
    }

    public function testNullControl(): void
    {
        $this->assertEquals(1, null);
    }

    private function stop(): never
    {
        throw new \RuntimeException('expected');
    }
}
