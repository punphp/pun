<?php

namespace App;

use PHPUnit\Framework\TestCase;

final class ITest extends TestCase
{
    #[Override]
    protected function setUp(): void
    {
        parent::setUp();
    }

    #[\Override]
    protected function tearDown(): void
    {
        if (true) {
            parent::tearDown();
        }
    }

    #[\Foo, \OVERRIDE]
    protected function SETUP(): void
    {
        parent::setup();
    }
}
-----
<?php

namespace App;

use PHPUnit\Framework\TestCase;

final class ITest extends TestCase
{
    #[Override]
    protected function setUp(): void
    {
        parent::setUp();
    }

    #[\Override]
    protected function tearDown(): void
    {
        if (true) {
            parent::tearDown();
        }
    }

    #[\Foo]
    protected function SETUP(): void
    {
        parent::setup();
    }
}
