<?php

namespace Pun\Ia;

use PHPUnit\Framework\TestCase;

/**
 * @template T of Repo
 */
final class EdgesTest extends TestCase
{
    /** @var T */
    private $templated;

    public function testEdges(Repo $repo, Repo&Shape $both, string $name): void
    {
        $a = $repo->find();
        ${'a'}->name();
        $repo->take(${'a'});
        $this->templated->take($a);
        $both->take($a);
        $repo->$name($a);
        $a->name();
    }

    public function testDynamic(Repo $repo, string $n): void
    {
        $a = $repo->find();
        $$n = $repo->find();
        $repo->take($$n);
        $repo->take($a);
    }
}
-----
<?php

namespace Pun\Ia;

use PHPUnit\Framework\TestCase;

/**
 * @template T of Repo
 */
final class EdgesTest extends TestCase
{
    /** @var T */
    private $templated;

    public function testEdges(Repo $repo, Repo&Shape $both, string $name): void
    {
        $a = $repo->find();
        ${'a'}->name();
        $repo->take(${'a'});
        $this->assertInstanceOf(\Pun\Ia\Item::class, $a);
        $this->templated->take($a);
        $both->take($a);
        $repo->$name($a);
        $a->name();
    }

    public function testDynamic(Repo $repo, string $n): void
    {
        $a = $repo->find();
        $$n = $repo->find();
        $repo->take($$n);
        $this->assertInstanceOf(\Pun\Ia\Item::class, $a);
        $repo->take($a);
    }
}
