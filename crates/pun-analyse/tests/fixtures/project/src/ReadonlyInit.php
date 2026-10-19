<?php

namespace ReadonlyInit;

final class ByRef
{
    /** @readonly */
    public array $matches;

    public readonly array $sorted;

    /** @readonly */
    public array $late;

    public function __construct()
    {
        preg_match('/a/', 'a', $this->matches);
        preg_match('/a/', 'a', $this->matches);
        $this->sorted = [2, 1];
        sort($this->sorted);
        $this->assignLate();
    }

    private function assignLate(): void
    {
        $this->late = [];
    }

    public function after(): void
    {
        echo count($this->matches);
    }
}

trait Construct
{
    public function __construct(public readonly int $fromTrait)
    {
    }
}

class Aliased
{
    use Construct {
        Construct::__construct as private traitConstruct;
    }

    public function __construct(public readonly string $own)
    {
        $this->traitConstruct(1);
    }
}

class AliasedUncalled
{
    use Construct {
        __construct as private traitConstruct;
    }

    public function __construct()
    {
    }
}
