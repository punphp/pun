<?php

namespace CoverageCallableTags;

/**
 * @param-immediately-invoked-callable $a
 * @phpstan-param-later-invoked-callable $a
 * @param-later-invoked-callable $b
 * @param-immediately-invoked-callable $c
 * @param-immediately-invoked-callable $missing
 * @param-later-invoked-callable
 */
function tags(int $a = null, $b = null, string $c = 'x'): void
{
}

/**
 * no tags
 */
function untagged(int $a): void
{
}
