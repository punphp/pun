<?php

final class KTest extends \PHPUnit\Framework\TestCase {
    public function testEmpty() {}
}

trait KTrait
{
    public function testEmpty()
    {
    }
}

$x = new #[Foo] readonly class (
    // c
    1,
) extends \PHPUnit\Framework\TestCase implements \Countable, \Stringable {
    // before
    public function testEmpty() {}
};
-----
<?php

final class KTest extends \PHPUnit\Framework\TestCase
{
}

trait KTrait
{
}

$x = new #[Foo]
readonly class(1) extends \PHPUnit\Framework\TestCase implements \Countable, \Stringable
{
};
