<?php

use PHPUnit\Framework\TestCase;

final class ATest extends TestCase
{
    #[\Override, Foo]
    protected function setUp(): void
    {
        parent::setUp();

        $some = 'value';
    }

    #[Bar]
    #[\Override]
    protected function TearDown(): void
    {
        // comment
        PARENT::tearDOWN();

        $x = 1; // trailing
    }

    #[Override]
    #[\Override]
    public function test(): void
    {
        parent::test();
    }
}
-----
<?php

use PHPUnit\Framework\TestCase;

final class ATest extends TestCase
{
    #[Foo]
    protected function setUp(): void
    {
        parent::setUp();

        $some = 'value';
    }

    #[Bar]
    protected function TearDown(): void
    {
        // comment
        PARENT::tearDOWN();

        $x = 1; // trailing
    }

    #[Override]
    #[\Override]
    public function test(): void
    {
        parent::test();
    }
}
