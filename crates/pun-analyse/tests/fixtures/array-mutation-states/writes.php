<?php declare(strict_types = 1);

namespace ArrayMutationStates;

/** @param array<string, int> $values */
function generalWrite(array $values, string $name): void
{
    $values[$name] = 42;
    \PHPStan\dumpType($values);
    $values[''] = 43;
    \PHPStan\dumpType($values);
    $values['keep'] = 44;
    \PHPStan\dumpType($values);
    $values[$name] = 45;
    \PHPStan\dumpType($values);
    echo 'written';
}

function constantShapeWrite(string $name): void
{
    $values = ['keep' => 1, 'other' => 2];
    $values[$name] = 3;
    \PHPStan\dumpType($values);
    echo 'shape';
}

/**
 * @param array<int|string, int> $values
 * @param int|'anchor' $name
 */
function mixedKeyWrite(array $values, int|string $name): void
{
    $values[$name] = 7;
    \PHPStan\dumpType($values);
    echo 'mixed key';
}

final class Registry
{
    /** @var array<string, int> */
    private array $values = [];

    public function add(string $name, bool $overwrite): void
    {
        $name = strtolower($name);
        \PHPStan\dumpType($name);
        if (!$overwrite and isset($this->values[$name])) {
            throw new \InvalidArgumentException();
        }
        \PHPStan\dumpType($name);
        \PHPStan\dumpType($this->values);
        $this->values[$name] = 42;
        \PHPStan\dumpType($this->values);
    }

    public function literal(): void
    {
        $this->values[''] = 43;
        \PHPStan\dumpType($this->values);
        $this->values['keep'] = 44;
        \PHPStan\dumpType($this->values);
    }
}
