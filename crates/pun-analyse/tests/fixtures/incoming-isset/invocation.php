<?php

namespace IssetInvocation;

function variables(bool $choose, ?string $nullable, string $defined): void
{
    var_dump(isset($nullable));
    var_dump(isset($defined));
    $null = null;
    var_dump(isset($null));
    var_dump(isset($missing));

    if ($choose) {
        $maybe = 42;
    }
    var_dump(isset($maybe));
    var_dump(isset(
        $nullable,
        $defined,
        $missingLater
    ));
    var_dump(isset($nullable, $maybe));
    var_dump(isset($_SESSION));
}

/**
 * @param array{present: int, nullable: ?int, nested: array{leaf: int}} $shape
 * @param array{present: int}|null $nullableShape
 */
function offsets(array $shape, ?array $nullableShape): void
{
    var_dump(isset($shape['present']));
    var_dump(isset($shape['missing']));
    var_dump(isset($shape['nullable']));
    var_dump(isset($shape['nested']['leaf']));
    var_dump(isset($nullableShape['present']));
    var_dump(isset($missingArray['key']));
}

class Properties
{
    public int $present = 1;
    public ?int $nullable = null;
    public static int $staticPresent = 1;

    public function initialized(): void
    {
        $this->present = 2;
        var_dump(isset($this->present));
    }
}

function properties(Properties $object): void
{
    var_dump(isset($object->present));
    var_dump(isset($object->nullable));
    var_dump(isset(Properties::$staticPresent));
    var_dump(isset($missingObject->present));
}

function repeatedOperands(?int $value, bool $flag): void
{
    if ($flag) {
        $maybe = 1;
    }
    var_dump(isset($value, $value));
    var_dump(isset($maybe, $maybe));
}
