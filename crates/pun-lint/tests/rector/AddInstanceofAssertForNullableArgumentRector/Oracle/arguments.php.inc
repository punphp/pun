<?php

namespace Pun\Ia;

use PHPUnit\Framework\TestCase;

final class ArgumentsTest extends TestCase
{
    public function testParams(Repo $repo): void
    {
        $a = $repo->find();
        $repo->takeNullable($a);
        $repo->takeAny($a);
        $repo->take($a);
        $repo->take($a);
    }

    public function testDoc(Repo $repo): void
    {
        $a = $repo->find();
        $repo->takeDoc($a);
    }

    public function testKind(Repo $repo): void
    {
        $item = $repo->find();
        $kind = $item->kind();
        $repo->takeKind($kind);
    }

    public function testTwo(Repo $repo): void
    {
        $a = $repo->find();
        $repo->takeTwo(1, $a);
        $repo->takeTwo($a, $a);
    }

    public function testTemplate(Repo $repo): void
    {
        $a = $repo->find();
        $repo->takeTemplate($a);
    }

    public function testAsserts(Repo $repo): void
    {
        $a = $repo->find();
        $repo->assertItem($a);
        $repo->shouldItem($a);
        $repo->ShouldItem($a);
    }

    public function testNested(Repo $repo): void
    {
        $a = $repo->find();
        $b = $repo->find();
        $repo->takeAny($repo->take($b));
        $repo->take($a);
    }

    public function testThisCall(Repo $repo): void
    {
        $a = $repo->find();
        $this->process($a);
        $this->processAny($a);
    }

    public function testUnknownReceiver(Repo $repo, $unknown): void
    {
        $a = $repo->find();
        $unknown->take($a);
        $repo->undefined($a);
        $repo->take();
        $repo->take(...[$a]);
    }

    public function testUncollected(Repo $repo, ?Item $param): void
    {
        $a = $repo->find();
        $repo->take($param);
        $repo->take($a);
    }

    private function process(Item $item): void
    {
    }

    private function processAny($item): void
    {
    }
}
-----
<?php

namespace Pun\Ia;

use PHPUnit\Framework\TestCase;

final class ArgumentsTest extends TestCase
{
    public function testParams(Repo $repo): void
    {
        $a = $repo->find();
        $this->assertInstanceOf(\Pun\Ia\Item::class, $a);
        $repo->takeNullable($a);
        $repo->takeAny($a);
        $repo->take($a);
        $repo->take($a);
    }

    public function testDoc(Repo $repo): void
    {
        $a = $repo->find();
        $this->assertInstanceOf(\Pun\Ia\Item::class, $a);
        $repo->takeDoc($a);
    }

    public function testKind(Repo $repo): void
    {
        $item = $repo->find();
        $kind = $item->kind();
        $this->assertInstanceOf(\Pun\Ia\Kind::class, $kind);
        $repo->takeKind($kind);
    }

    public function testTwo(Repo $repo): void
    {
        $a = $repo->find();
        $this->assertInstanceOf(\Pun\Ia\Item::class, $a);
        $repo->takeTwo(1, $a);
        $repo->takeTwo($a, $a);
    }

    public function testTemplate(Repo $repo): void
    {
        $a = $repo->find();
        $this->assertInstanceOf(\Pun\Ia\Item::class, $a);
        $repo->takeTemplate($a);
    }

    public function testAsserts(Repo $repo): void
    {
        $a = $repo->find();
        $repo->assertItem($a);
        $repo->shouldItem($a);
        $this->assertInstanceOf(\Pun\Ia\Item::class, $a);
        $repo->ShouldItem($a);
    }

    public function testNested(Repo $repo): void
    {
        $a = $repo->find();
        $b = $repo->find();
        $this->assertInstanceOf(\Pun\Ia\Item::class, $b);
        $repo->takeAny($repo->take($b));
        $this->assertInstanceOf(\Pun\Ia\Item::class, $a);
        $repo->take($a);
    }

    public function testThisCall(Repo $repo): void
    {
        $a = $repo->find();
        $this->assertInstanceOf(\Pun\Ia\Item::class, $a);
        $this->process($a);
        $this->processAny($a);
    }

    public function testUnknownReceiver(Repo $repo, $unknown): void
    {
        $a = $repo->find();
        $unknown->take($a);
        $repo->undefined($a);
        $repo->take();
        $repo->take(...[$a]);
    }

    public function testUncollected(Repo $repo, ?Item $param): void
    {
        $a = $repo->find();
        $repo->take($param);
        $this->assertInstanceOf(\Pun\Ia\Item::class, $a);
        $repo->take($a);
    }

    private function process(Item $item): void
    {
    }

    private function processAny($item): void
    {
    }
}
