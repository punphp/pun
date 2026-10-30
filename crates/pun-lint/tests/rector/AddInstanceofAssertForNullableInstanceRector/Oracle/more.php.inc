<?php

namespace Pun\Ia;

use PHPUnit\Framework\TestCase;

final class MoreTest extends TestCase
{
    public function testParamFirst(Repo $repo, ?Item $p): void
    {
        // uses the parameter first
        $p->name();
        $p = $repo->find();
    }

    public function testArgs(Repo $repo): void
    {
        $a = $repo->find();
        $repo->find($a);
        $kind = $a->kind();
        $kind->label($a);
        $repo->take($a);
    }

	public function testTabs(Repo $repo): void
	{
		$a = $repo->find();
		$a->name();
	}
}
-----
<?php

namespace Pun\Ia;

use PHPUnit\Framework\TestCase;

final class MoreTest extends TestCase
{
    public function testParamFirst(Repo $repo, ?Item $p): void
    {
        $this->assertInstanceOf(\Pun\Ia\Item::class, $p);
        // uses the parameter first
        $p->name();
        $p = $repo->find();
    }

    public function testArgs(Repo $repo): void
    {
        $a = $repo->find();
        $repo->find($a);
        $this->assertInstanceOf(\Pun\Ia\Item::class, $a);
        $kind = $a->kind();
        $this->assertInstanceOf(\Pun\Ia\Kind::class, $kind);
        $kind->label($a);
        $repo->take($a);
    }

	public function testTabs(Repo $repo): void
	{
		$a = $repo->find();
        $this->assertInstanceOf(\Pun\Ia\Item::class, $a);
		$a->name();
	}
}
