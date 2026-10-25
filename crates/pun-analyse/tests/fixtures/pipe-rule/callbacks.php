<?php declare(strict_types=1);

namespace PipeRule;

function reference(string &$text): string { return $text; }
function value(string $text): string { return $text; }
function variadic(string &...$texts): string { return ''; }
function later(string $text, string &$second = ''): string { return $text; }
function none(): string { return ''; }

/**
 * @param callable(string &): string $unnamed
 * @param callable(string &$named): string $named
 * @param callable(string &...): string $variadic
 * @param callable-string $callableString
 */
function callbacks(callable $unnamed, callable $named, callable $variadic, callable $unknown, mixed $mixed, string $callableString): void
{
    'text' |> $unnamed;
    'text' |> $named;
    'text' |> $variadic;
    'text' |> $unknown;
    'text' |> $mixed;
    'text' |> 123;
    'text' |> 'not_a_function';
    'text' |> reference(...);
    'text' |> 'PipeRule\reference';
    'text' |> variadic(...);
    'text' |> later(...);
    'text' |> none(...);
    'text' |> value(...);
    'text' |> (fn (string &$arrow): string => $arrow);
    'text' |> function (string &$closure): string { return $closure; };
    'text' |> function (string $first, string &$second = ''): string { return $first; };
    'text' |> (fn (): string => '');
    'text'
        |>
            function (string &$multiline): string { return $multiline; };
    'text' |> (
        function (string &$parenthesized): string { return $parenthesized; }
    );
    [2, 1] |> sort(...);
    'text' |> trim(...);
    'text' |> $callableString;
    [2, 1] |> 'sort';
}
