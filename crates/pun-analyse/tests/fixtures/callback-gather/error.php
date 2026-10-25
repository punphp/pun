<?php declare(strict_types=1);

/**
 * @template T
 * @param callable(array): T $callback
 * @return T
 */
function forwardForError(array $input, callable $callback)
{
    return $callback($input);
}

function failGather(): array
{
    return [];
}

$result = forwardForError([], function (array $entry): array {
    return failGather();
});
