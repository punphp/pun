<?php

namespace App\Tests;

use PHPUnit\Framework\Attributes as A;
use PHPUnit\Framework\Attributes\DataProvider;

interface HasTests
{
    public function testInInterface(): void;
}

trait SharedTests
{
	public static function Test_tabbed(): void {}
}

enum Kind
{
    case A;

    public function testEnum(): void {}
}

final class CasesTest
{
    /**
     * A doc comment.
     */
    #[DataProvider('provide')]
    public function testWithProvider(int $a): void {}

    #[A\Test]
    public function testAliased(): void {}

    #[\phpunit\framework\attributes\TEST]
    public function testCaseInsensitive(): void {}

    #[Test]
    public function testUnimportedTest(): void {}

    public function Test(): void {}

    public function test_(): void {}

    public function testX(): void {}

    public function test1(): void {}

    public function testé(): void {}

    public function tester(): void {}

    public function test__double(): void {}

    protected function testProtected(): void {}

    private  function   testSpaces(): void {}

    public function provide(): array { return []; }
}

$anonymous = new class {
    public function testAnonymous(): void {}
};

final class UpperCasesTest
{
    public function Test_(): void {}

    public function TestX(): void {}

    public function testTestTwice(): void {}
}

function testFunction(): void {}
