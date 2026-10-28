<?php
namespace UnpackOrder;

function fixed(mixed $before = null, int $left = 0, int $right = 0): void {}
function variadic(mixed ...$values): void {}
/**
 * @template T
 * @param T ...$values
 * @return T
 */
function generic(...$values): mixed { return $values[0]; }
function emptyTarget(): void {}
function readsExtra(): void { func_get_args(); }
function reference(mixed &...$values): void {}
function boundarySkipped(): array { return [1]; }

function ordered(): void
{
    $arguments = ['left' => 1];
    fixed(...$arguments, before: $arguments = ['right' => 1]);
    $arguments = ['right' => 1];
    fixed($arguments = ['left' => 1], ...$arguments);
    $arguments = ['left' => 1];
    variadic(...$arguments, before: $arguments = ['right' => 1]);
    $arguments = ['right' => 1];
    variadic($arguments = ['left' => 1], ...$arguments);
    $arguments = [1];
    generic(...$arguments, before: $arguments = [2]);
    $arguments = [1];
    variadic(...$arguments, ...($arguments = [2]), ...$arguments);
    emptyTarget(...boundarySkipped());
    readsExtra(...boundarySkipped());
    variadic(...boundarySkipped());
    generic(...boundarySkipped());
    $arguments = [1];
    reference(...$arguments);
}
