<?php

final class JTest extends \PHPUnit\Framework\TestCase  implements \Countable {
    public function __construct() {}
}

$x = new class (1, 2) extends \PHPUnit\Framework\TestCase {
    public function __construct() {}
    // tail
};
-----
<?php

final class JTest extends \PHPUnit\Framework\TestCase implements \Countable
{
}

$x = new class (1, 2) extends \PHPUnit\Framework\TestCase {
    // tail
};
