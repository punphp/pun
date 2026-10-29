<?php
declare(strict_types=1);

/** @template-contravariant T */
class RelationContra {}

function relationClosureProjection(): void {
    $callback = static fn(int $value): int => $value;
    if ($callback instanceof Closure) {
        $answer = $callback(7);
        echo $answer, $callback('bad');
    }
}

/** @template T */
class RelationInvariant {}
