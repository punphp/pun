<?php

final class LTest extends \PHPUnit\Framework\TestCase
{
    #[\Override]
    protected function setUp(): void
    {
        /**
         * Doc.
         */
        parent::setUp();
        // one

        // two
        $a = 1;

        /* block
           comment */
        $b = <<<TXT
          keep
        TXT;
    }
}

$x = new #[Foo] class extends \PHPUnit\Framework\TestCase {
    public function __construct() {}
};
-----
<?php

final class LTest extends \PHPUnit\Framework\TestCase
{
    protected function setUp(): void
    {
        /**
         * Doc.
         */
        parent::setUp();
        // one
        // two
        $a = 1;
        /* block
           comment */
        $b = <<<TXT
          keep
        TXT;
    }
}

$x = new #[Foo] class extends \PHPUnit\Framework\TestCase {
    public function __construct() {}
};
