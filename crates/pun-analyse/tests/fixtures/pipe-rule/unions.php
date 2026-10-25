<?php declare(strict_types=1);

namespace PipeRuleUnions;

final class AValue
{
    public function __invoke(string $value): string { return $value; }
}

final class ZReference
{
    public function __invoke(string &$reference): string { return $reference; }
}

/**
 * @param (callable(string &): string)|null $nullable
 * @param (callable(string &): string)|int $partlyCallable
 * @param (callable(string): string)|(callable(string &): string) $bothCallable
 * @param ZReference|MissingClass $missing
 */
function examples(AValue|ZReference $objects, ?ZReference $nullableObject, callable|null $nullable, callable|int $partlyCallable, callable $bothCallable, $missing, bool $choose): void
{
    'text' |> $objects;
    'text' |> $nullableObject;
    'text' |> $nullable;
    'text' |> $partlyCallable;
    'text' |> $bothCallable;
    'text' |> $missing;
    'text' |> ($choose
        ? function (string $value): string { return $value; }
        : function (string &$reference): string { return $reference; });
}
