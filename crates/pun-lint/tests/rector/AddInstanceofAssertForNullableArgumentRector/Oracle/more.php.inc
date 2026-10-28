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
        $m = $repo->findUnknown();
        $m->take($a);
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
        $m = $repo->findUnknown();
        $m->take($a);
        $this->assertInstanceOf(\Pun\Ia\Item::class, $a);
        $repo->take($a);
    }

	public function testTabs(Repo $repo): void
	{
		$a = $repo->find();
		$a->name();
	}
}
