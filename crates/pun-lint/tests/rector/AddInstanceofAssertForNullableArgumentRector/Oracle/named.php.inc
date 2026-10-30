<?php

namespace Pun\Ia;

use PHPUnit\Framework\TestCase;

final class NamedTest extends TestCase
{
    public function testNamed(Repo $repo): void
    {
        $a = $repo->find();
        $repo->take(item: $a);
    }

    public function testNamedSecond(Repo $repo): void
    {
        $a = $repo->find();
        $repo->takeTwo(1, item: $a);
    }
}
-----
<?php

namespace Pun\Ia;

use PHPUnit\Framework\TestCase;

final class NamedTest extends TestCase
{
    public function testNamed(Repo $repo): void
    {
        $a = $repo->find();
        $this->assertInstanceOf(\Pun\Ia\Item::class, $a);
        $repo->take(item: $a);
    }

    public function testNamedSecond(Repo $repo): void
    {
        $a = $repo->find();
        $this->assertInstanceOf(\Pun\Ia\Item::class, $a);
        $repo->takeTwo(1, item: $a);
    }
}
