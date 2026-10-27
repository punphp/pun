<?php

namespace App\Tests\Ctor;

use PHPUnit\Framework\TestCase;

final class StatementsTest extends TestCase
{
    private $value;

    public function __construct()
    {
        parent::__construct();
        if (true) {
            $this->value = 1;
        }
        // a comment between
        $this->value = 2;
    }
}

final class AttributedSetUpTest extends TestCase
{
    private $value;

    public function __construct()
    {
        $this->value = 1;
    }

    #[\Override]
    #[\Deprecated]
    protected function &setUp(): void
    {
    }
}

final class OnlyParentTest extends TestCase
{
    public function __construct()
    {
        parent::__construct();
    }
}

final class OnlyParentWithSetUpTest extends TestCase
{
    public function __construct()
    {
        parent::__construct();
    }

    protected function setUp(): void
    {
    }
}
-----
<?php

namespace App\Tests\Ctor;

use PHPUnit\Framework\TestCase;

final class StatementsTest extends TestCase
{
    private $value;

    protected function setUp(): void
    {
        if (true) {
            $this->value = 1;
        }
        // a comment between
        $this->value = 2;
    }
}

final class AttributedSetUpTest extends TestCase
{
    private $value;

    #[\Override]
    #[\Deprecated]
    protected function &setUp(): void
    {
        $this->value = 1;
    }
}

final class OnlyParentTest extends TestCase
{
    protected function setUp(): void
    {
    }
}

final class OnlyParentWithSetUpTest extends TestCase
{
    protected function setUp(): void
    {
    }
}
