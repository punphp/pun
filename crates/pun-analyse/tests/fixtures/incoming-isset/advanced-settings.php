<?php

declare(strict_types=1);

namespace AdvancedIssetSettings;

function variables(string $defined, ?int $nullable, bool $choose): void
{
    $null = null;
    if ($choose) {
        $maybe = 1;
    }
    var_dump(isset($defined), isset($nullable), isset($null), isset($missing), isset($maybe));
    var_dump(isset($nullable, $defined, $laterMissing));
    var_dump($defined ?? 0, $nullable ?? 0, $null ?? 0, $missingCoalesce ?? 0);
    $defined ??= 'fallback';
    $nullable ??= 0;
    $neverAssigned ??= 0;
}

/** @param array{required: int, nullable: ?int, optional?: int, nested: array{leaf: int}} $shape */
function offsets(array $shape): void
{
    var_dump(isset($shape['required']), isset($shape['nullable']), isset($shape['optional']));
    var_dump(isset($shape['absent']), isset($shape['nested']['leaf']));
    var_dump($shape['required'] ?? 0, $shape['nullable'] ?? 0, $shape['optional'] ?? 0);
    var_dump($shape['absent'] ?? 0, $shape['nested']['leaf'] ?? 0);
    $shape['required'] ??= 0;
    $shape['nullable'] ??= 0;
    $shape['optional'] ??= 0;
}

/** @param array{required: int} $shape */
function absentAssignment(array $shape): void
{
    $shape['absentAssignment'] ??= 0;
    var_dump($shape);
}

/** @param array{nested: array{leaf: int}|null}|null $shape */
function nullableChain(?array $shape): void
{
    var_dump(isset($shape['nested']['leaf']));
    var_dump($shape['nested']['leaf'] ?? 0);
    $shape['nested']['leaf'] ??= 0;
}

class Properties
{
    public int $defaulted = 1;
    public ?int $nullable = null;
    public int $uninitialized;
    public int $initialized;

    public function initialize(): void
    {
        $this->initialized = 1;
        var_dump(isset($this->initialized));
        var_dump($this->initialized ?? 0);
        $this->initialized ??= 0;
    }
}

function properties(Properties $object): void
{
    var_dump(isset($object->defaulted), isset($object->nullable), isset($object->uninitialized));
    var_dump($object->defaulted ?? 0, $object->nullable ?? 0, $object->uninitialized ?? 0);
    $object->defaulted ??= 0;
    $object->nullable ??= 0;
    $object->uninitialized ??= 0;
    var_dump(isset($missingObject->defaulted));
    var_dump($missingCoalesceObject->defaulted ?? 0);
}

function number(): int
{
    return 1;
}

function expressions(): void
{
    var_dump(number() ?? 0);
    var_dump(null ?? 0);
    var_dump((new Properties())->defaulted ?? 0);
}

function unrelated(Properties $object): void
{
    $object->defaulted = 'wrong';
    strlen(123);
}

/** @param array{required: object} $shape */
function undefinedInner(array $shape): void
{
    // checkUndefinedInner() still reports missing offsets with advanced checks disabled.
    var_dump(isset($shape['missing'][0]));
    var_dump($shape['missing'][0] ?? 0);
}
