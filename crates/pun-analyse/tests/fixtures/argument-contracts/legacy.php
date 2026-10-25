<?php declare(strict_types=1);

namespace ArgumentLegacy;

/** @param-closure-this \stdClass $callback */
function bind(\Closure $callback): void {}

/**
 * @template T
 * @param T $input
 * @param value-of<T> $value
 */
function value($input, $value): void {}

bind(static function (): void {});
value(0, 0);
