<?php declare(strict_types=1);

namespace ConditionalCallbackGather;

interface Item {}
final class First implements Item {}
final class Second implements Item {}

/**
 * @template T
 * @param class-string<T>|string $class
 * @return ($class is class-string<T> ? T : mixed)
 */
function make(string $class) {
    return new $class();
}

/**
 * @template T
 * @param class-string<T> $class
 * @return ($class is not class-string<T> ? mixed : T)
 */
function negated(string $class) {
    return new $class();
}

/**
 * @template T
 * @param class-string<T>|string $class
 * @return ($class is class-string<T> ? T : int)
 */
function fallback(int $unused, string $class) {
    return 13;
}

/**
 * @template T
 * @param class-string<T> $class
 * @return T
 */
function ordinary(string $class) {
    return new $class();
}

/** @param Item[] $items */
function receive(array $items): void {}

/** @param non-empty-list<class-string<First>|class-string<Second>> $classes */
function examples(string $unknown, array $classes): void {
    \PHPStan\dumpType(make(First::class));
    \PHPStan\dumpType(make(class: Second::class));
    \PHPStan\dumpType(make($unknown));
    \PHPStan\dumpType(fallback(7, Second::class));
    \PHPStan\dumpType(fallback(class: First::class, unused: 7));
    \PHPStan\dumpType(fallback(7, $unknown));
    \PHPStan\dumpType(negated(First::class));
    \PHPStan\dumpType(ordinary(Second::class));

    \PHPStan\dumpType(array_map(make(...), $classes));
    \PHPStan\dumpType(array_map(ordinary(...), $classes));
    \PHPStan\dumpType(array_map(negated(...), $classes));
    \PHPStan\dumpType(array_map(static fn ($class) => make($class), $classes));
    receive(array_map(make(...), $classes));

    $callable = make(...);
    \PHPStan\dumpType($callable(Second::class));
    \PHPStan\dumpType($callable($unknown));
    \PHPStan\dumpType(17);
}
