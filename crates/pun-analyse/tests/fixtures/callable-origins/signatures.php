<?php declare(strict_types=1);

namespace CallableOrigins;

use function PHPStan\dumpType;

/**
 * @param callable(string &): string $reference
 * @param callable(string &$named): string $named
 * @param callable(string &...): string $variadic
 * @param callable(string, int &): bool $later
 * @param (callable(string &): string)|null $nullable
 * @param (callable(string &): string)|int $partial
 * @param (callable(string): string)|(callable(string &): string) $both
 */
function signatures(callable $reference, callable $named, callable $variadic, callable $later, ?callable $nullable, callable|int $partial, callable $both): void
{
    dumpType($reference, $named, $variadic, $later, $nullable, $partial, $both);
    $input = 'text'; $input |> $reference;
    $input = 'text'; $input |> $named;
    $input = 'text'; $input |> $variadic;
    $input = 'text'; $input |> $later;
    $input = 'text'; $input |> $nullable;
    $input = 'text'; $input |> $partial;
    $input = 'text'; $input |> $both;
}

/** @return callable(string &): string */
function factory(): callable { return static fn(string &$input): string => $input; }

function returned(): void
{
    $callback = factory();
    dumpType($callback);
    $input = 'text'; $input |> $callback;
}

class Factory
{
    /** @return callable(string &): string */
    public function callback(): callable { return static fn(string &$input): string => $input; }
}

function method(Factory $factory): void
{
    $callback = $factory->callback();
    dumpType($callback);
    $input = 'text'; $input |> $callback;
}

function annotation(callable $unknown): void
{
    /** @var callable(string &): string $callback */
    $callback = $unknown;
    dumpType($callback);
    $input = 'text'; $input |> $callback;
}
