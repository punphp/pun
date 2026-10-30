<?php

namespace MapperIntersectionContracts;

use MapperIntersectionContracts\Ambiguous as AmbiguousAlias;
use MapperIntersectionContracts\Proven as ProvenAlias;

class Ambiguous
{
    public function inspect(self&\Countable $other): void
    {
        /** @var AmbiguousAlias */
        $ambiguous = $other;
        echo $ambiguous::class;

        /** @var AmbiguousAlias&\Countable */
        $complete = $other;
        echo $complete::class;
    }

    public function plain(self $other): void
    {
        /** @var AmbiguousAlias */
        $plain = $other;
        echo $plain::class;
    }

    public function alternative((self&\Countable)|\stdClass $other): void
    {
        /** @var AmbiguousAlias|\stdClass */
        $alternative = $other;
        echo $alternative::class;
    }
}

class Proven implements \Countable
{
    public function count(): int
    {
        return 0;
    }

    public function inspect(self&\Countable $other): void
    {
        /** @var ProvenAlias */
        $proven = $other;
        echo $proven::class;
    }
}

class StaticReceiver
{
    /** @return static&\Countable */
    public function refined(): static
    {
        throw new \LogicException();
    }

    public function inspect(): void
    {
        /** @var StaticReceiver */
        $refined = $this->refined();
        echo $refined::class;

        /** @var StaticReceiver */
        $same = $this;
        echo $same::class;
    }
}
-----
<?php

namespace MapperIntersectionContracts;

use MapperIntersectionContracts\Ambiguous as AmbiguousAlias;
use MapperIntersectionContracts\Proven as ProvenAlias;

class Ambiguous
{
    public function inspect(self&\Countable $other): void
    {
        /** @var AmbiguousAlias */
        $ambiguous = $other;
        echo $ambiguous::class;

        $complete = $other;
        echo $complete::class;
    }

    public function plain(self $other): void
    {
        $plain = $other;
        echo $plain::class;
    }

    public function alternative((self&\Countable)|\stdClass $other): void
    {
        /** @var AmbiguousAlias|\stdClass */
        $alternative = $other;
        echo $alternative::class;
    }
}

class Proven implements \Countable
{
    public function count(): int
    {
        return 0;
    }

    public function inspect(self&\Countable $other): void
    {
        $proven = $other;
        echo $proven::class;
    }
}

class StaticReceiver
{
    /** @return static&\Countable */
    public function refined(): static
    {
        throw new \LogicException();
    }

    public function inspect(): void
    {
        /** @var StaticReceiver */
        $refined = $this->refined();
        echo $refined::class;

        /** @var StaticReceiver */
        $same = $this;
        echo $same::class;
    }
}
