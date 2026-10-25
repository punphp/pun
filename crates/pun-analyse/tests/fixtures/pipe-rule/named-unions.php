<?php declare(strict_types=1);

namespace PipeRuleNamedUnions;

final class AValue
{
    public function __invoke(string $value): string { return $value; }
}

final class ZReference
{
    public function __invoke(string &$reference): string { return $reference; }
}

/**
 * @param (callable(string &$input): string)|null $nullable
 * @param (callable(string &$input): string)|int $partlyCallable
 * @param (callable(string $input): string)|(callable(string &$input): string) $bothCallable
 * @param ZReference|MissingClass $missing
 */
function examples(AValue|ZReference $objects, ?ZReference $nullableObject, callable|null $nullable, callable|int $partlyCallable, callable $bothCallable, $missing, mixed $explicit, $implicit): void
{
    'text' |> $objects;
    'text' |> $nullableObject;
    'text' |> $nullable;
    'text' |> $partlyCallable;
    'text' |> $bothCallable;
    'text' |> $missing;
    'text' |> $explicit;
    'text' |> $implicit;
}
