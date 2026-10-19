<?php

namespace PunPrintfPipe;

use function sprintf as render;

function formats(string $unknown): void
{
    '%s' |> render(...);
    '%q' |> \printf(...);
    'plain' |> \sprintf(...);
    '%s' |> \vsprintf(...);
    '%q' |> \vprintf(...);
    $unknown |> \printf(...);
    '%s' |> \sscanf(...);
    '%s' |> \fscanf(...);
    '%s' |>
        \printf(...);
    $callable = 'printf';
    '%s' |> $callable;
}

namespace PunShadowPipe;

function sprintf(...$args): string { return ''; }
'%s' |> sprintf(...);
