<?php

namespace App\ConstantArrays;

function a(): array
{
    /** @var list<int> $constant */
    $constant = [1, 2];

    return (array) $constant;
}

function b(): array
{
    $constant = [1, 2];

    return (array) $constant;
}

function c(): array
{
    /** @var array<int> $constant */
    $constant = [1, 2];

    return (array) $constant;
}

class M
{
    public function a(): array
    {
        /** @var list<int> $constant */
        $constant = [1, 2];

        return (array) $constant;
    }

    public function b(array $shape): array
    {
        /** @var list<int> $constant */
        $constant = [1, 2];

        return [(array) $shape, (array) $constant];
    }

    public function c(): array
    {
        /** @var list<int> $constant */
        $constant = [1, 2];

        return [(array) $constant];
    }
}
-----
<?php

namespace App\ConstantArrays;

function a(): array
{
    /** @var list<int> $constant */
    $constant = [1, 2];

    return (array) $constant;
}

function b(): array
{
    $constant = [1, 2];

    return $constant;
}

function c(): array
{
    /** @var array<int> $constant */
    $constant = [1, 2];

    return (array) $constant;
}

class M
{
    public function a(): array
    {
        /** @var list<int> $constant */
        $constant = [1, 2];

        return (array) $constant;
    }

    public function b(array $shape): array
    {
        /** @var list<int> $constant */
        $constant = [1, 2];

        return [$shape, (array) $constant];
    }

    public function c(): array
    {
        /** @var list<int> $constant */
        $constant = [1, 2];

        return [(array) $constant];
    }
}
