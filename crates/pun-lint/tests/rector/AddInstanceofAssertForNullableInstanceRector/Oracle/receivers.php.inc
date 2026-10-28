<?php

namespace Pun\Ia;

use PHPUnit\Framework\TestCase;

final class ReceiversTest extends TestCase
{
    public function testChain(Repo $repo): void
    {
        $item = $repo->find();
        $next = $item->next();
        $name = $next->name() . $next->name();
        $item->name();
    }

    public function testShapes(Repo $repo): void
    {
        $item = $repo->find();
        $kind = $item->kind();
        $either = $item->either();
        $count = $item->count();
        $self = $item->self();
        $both = $item->both();
        $sure = $item->sure();
        $missing = $repo->findUnknown();
        echo $kind->label(), $either->name(), $self->name(), $both->name(), $sure->name(), $missing->run();
    }

    public function testTwice(Repo $repo): void
    {
        $a = $repo->find();
        $a->name();
        $a = $repo->find();
        $a->name();
        $a->name();
    }

    public function testSameLine(Repo $repo): void
    {
        $a = $repo->find(); $a->name();
        // a comment
        $b = $a->next();
        /** @var Item|null $c */
        $c = $b->next();
        $c->name();
    }

    public function testFirst(Repo $repo): void
    {
        // first
        $late->name();
        $late = $repo->find();
    }

    public function testNested(Repo $repo): void
    {
        $item = $repo->find();
        $fn = function () use ($item) {
            return $item->name();
        };
        $other = ($repo->find());
        $other?->name();
        $other->name();
    }

    public function testNonExpression(Repo $repo): void
    {
        $item = $repo->find();
        if (true) {
        }
        $item->name();
    }

    public function testTrailingComment(Repo $repo): void
    {
        $item = $repo->find();
        $item->name();
        // done
    }

    public function testOne(Repo $repo): void
    {
        $item = $repo->find();
    }

    public function testLoop(Repo $repo, array $list): void
    {
        foreach ($list as $x) {
            $item = $repo->find();
            $item->name();
        }
        foreach ($list as $x):
            $item = $repo->find();
            $item->name();
        endforeach;
        foreach ($list as $x) {
            $item = $repo->find();
            if (true) {
            }
            $item->name();
        }
    }

    private function helper(Repo $repo): void
    {
        $item = $repo->find();
        $item->name();
    }
}

final class NotATest
{
    public function testThat(Repo $repo): void
    {
        $item = $repo->find();
        $item->name();
    }
}
-----
<?php

namespace Pun\Ia;

use PHPUnit\Framework\TestCase;

final class ReceiversTest extends TestCase
{
    public function testChain(Repo $repo): void
    {
        $item = $repo->find();
        $this->assertInstanceOf(\Pun\Ia\Item::class, $item);
        $next = $item->next();
        $this->assertInstanceOf(\Pun\Ia\Item::class, $next);
        $name = $next->name() . $next->name();
        $item->name();
    }

    public function testShapes(Repo $repo): void
    {
        $item = $repo->find();
        $kind = $item->kind();
        $either = $item->either();
        $count = $item->count();
        $self = $item->self();
        $both = $item->both();
        $sure = $item->sure();
        $missing = $repo->findUnknown();
        echo $kind->label(), $either->name(), $self->name(), $both->name(), $sure->name(), $missing->run();
    }

    public function testTwice(Repo $repo): void
    {
        $a = $repo->find();
        $this->assertInstanceOf(\Pun\Ia\Item::class, $a);
        $a->name();
        $a = $repo->find();
        $this->assertInstanceOf(\Pun\Ia\Item::class, $a);
        $a->name();
        $a->name();
    }

    public function testSameLine(Repo $repo): void
    {
        $a = $repo->find();
        $this->assertInstanceOf(\Pun\Ia\Item::class, $a); $a->name();
        // a comment
        $b = $a->next();
        $this->assertInstanceOf(\Pun\Ia\Item::class, $b);
        /** @var Item|null $c */
        $c = $b->next();
        $this->assertInstanceOf(\Pun\Ia\Item::class, $c);
        $c->name();
    }

    public function testFirst(Repo $repo): void
    {
        // first
        $late->name();
        $late = $repo->find();
    }

    public function testNested(Repo $repo): void
    {
        $item = $repo->find();
        $this->assertInstanceOf(\Pun\Ia\Item::class, $item);
        $fn = function () use ($item) {
            return $item->name();
        };
        $other = ($repo->find());
        $other?->name();
        $this->assertInstanceOf(\Pun\Ia\Item::class, $other);
        $other->name();
    }

    public function testNonExpression(Repo $repo): void
    {
        $item = $repo->find();
        if (true) {
        }
        $item->name();
    }

    public function testTrailingComment(Repo $repo): void
    {
        $item = $repo->find();
        $item->name();
        // done
    }

    public function testOne(Repo $repo): void
    {
        $item = $repo->find();
    }

    public function testLoop(Repo $repo, array $list): void
    {
        foreach ($list as $x) {
            $item = $repo->find();
            $this->assertInstanceOf(\Pun\Ia\Item::class, $item);
            $item->name();
        }
        foreach ($list as $x):
            $item = $repo->find();
            $this->assertInstanceOf(\Pun\Ia\Item::class, $item);
            $item->name();
        endforeach;
        foreach ($list as $x) {
            $item = $repo->find();
            if (true) {
            }
            $item->name();
        }
    }

    private function helper(Repo $repo): void
    {
        $item = $repo->find();
        $this->assertInstanceOf(\Pun\Ia\Item::class, $item);
        $item->name();
    }
}

final class NotATest
{
    public function testThat(Repo $repo): void
    {
        $item = $repo->find();
        $item->name();
    }
}
