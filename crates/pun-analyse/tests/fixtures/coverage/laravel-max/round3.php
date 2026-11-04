<?php

namespace LaravelMax\Round3;

/**
 * `!isset($a[$k]) || !is_array($a[$k])` then return: the post-if scope keeps the specified `$a[$k]`.
 *
 * @param array<string, int|array<int>> $a
 */
function offsetHolder(array $a, string $k): int
{
    if (! isset($a[$k]) || ! is_array($a[$k])) {
        return 0;
    }

    return $a[$k];
}

function mixedOffsetHolder(mixed $array, string $key): string
{
    if (! isset($array[$key]) || ! is_array($array[$key])) {
        $array[$key] = [];
    }

    return $array[$key];
}

/**
 * Calls through the scope class check its own templates in place.
 *
 * @template TKey of array-key
 * @template TValue
 */
class Items
{
    /** @param TValue $value */
    public function push($value): void
    {
    }

    /** @param iterable<TKey, TValue> $items */
    public static function make($items): void
    {
    }

    /** @param array<int> $numbers */
    public function fill(array $numbers): void
    {
        $this->push(1);
        $this->push($numbers);
        static::make($numbers);
        self::make([1]);
    }
}

/** @extends Items<int, string> */
class Strings extends Items
{
    public function add(): void
    {
        $this->push(1);
        parent::push(2);
    }
}

/**
 * @template T
 * @extends Items<int, T>
 */
class Generic extends Items
{
    public function add(): void
    {
        $this->push(1);
    }
}

class User
{
    public bool $verified = false;
}

/** array_filter() narrows by its callback's returned expression. */
class Filters
{
    /**
     * @param array<string, int|string|null> $values
     * @return array<string, string>
     */
    public function ints(array $values): array
    {
        return array_filter($values, fn ($value) => is_int($value));
    }

    /**
     * @param list<User|null> $users
     * @return array<int, User>
     */
    public function users(array $users): array
    {
        return array_filter($users, static fn (?User $user): bool => $user instanceof User);
    }

    /**
     * @param array<string, int|string|null> $values
     * @return array<string, int>
     */
    public function strings(array $values): array
    {
        return array_filter($values, 'is_string');
    }

    /**
     * @param array<string, int|null> $values
     * @return array<string, int>
     */
    public function keys(array $values): array
    {
        return array_filter($values, function ($value, $key) {
            return $value !== null && $key !== 'a';
        }, ARRAY_FILTER_USE_BOTH);
    }

    /** @return array{a: 1} */
    public function shape(): array
    {
        return array_filter(['a' => 1, 'b' => null], fn ($value) => $value !== null);
    }
}

/** @param true $verified */
function verified(bool $verified): void
{
    var_dump($verified);
}

/** A conjunction's falsey scope remembers that with its left side true, its right side is false. */
function links(int $count, User $user): void
{
    if ($count >= 10 && ! $user->verified) {
        return;
    }

    if ($count >= 20) {
        verified($user->verified);
    }
}

/** method_exists() of a mixed value is `(object&hasMethod(m))|class-string`. */
class Events
{
    public mixed $event = null;

    public function name(): mixed
    {
        return method_exists($this->event, 'broadcastAs')
            ? $this->event->broadcastAs()
            : null;
    }
}

/**
 * PDO::prepare() returns a benevolent union.
 *
 * @throws \PDOException
 */
function prepare(\PDO $pdo, string $query): bool
{
    $statement = $pdo->prepare($query);

    return $statement->execute();
}

/**
 * Iterator::key() and current() return the bare templates.
 *
 * @template TKey of array-key
 * @template TValue
 */
class Lazy
{
    /** @param \Closure(): \Generator<TKey, TValue, mixed, void> $source */
    final public function __construct(public \Closure $source)
    {
    }

    /** @return \Iterator<TKey, TValue> */
    public function getIterator(): \Iterator
    {
        return ($this->source)();
    }

    /** @return static */
    public function skip(int $count)
    {
        return new static(function () use ($count) {
            $iterator = $this->getIterator();

            while ($iterator->valid() && $count-- > 0) {
                $iterator->next();
            }

            yield $iterator->key() => $iterator->current();
        });
    }
}

/** @return ($string is null ? int : string) */
function pick(?string $string = null)
{
    return $string === null ? 1 : $string;
}

/** Superglobals are `array<array-key, mixed>`: an isset() offset is `mixed~null`, which is never null. */
function server(): string
{
    if (isset($_SERVER['APP_NAME'])) {
        return pick($_SERVER['APP_NAME']);
    }

    return '';
}

/** `+` with a mixed operand is `(float|int)`. */
function sum(int $i, mixed $m): string
{
    return $i + $m;
}

/**
 * foreach over `$this` keeps the class's own templates.
 *
 * @template TKey of array-key
 * @template TValue
 * @implements \IteratorAggregate<TKey, TValue>
 */
class Bag implements \IteratorAggregate
{
    /** @return \Traversable<TKey, TValue> */
    public function getIterator(): \Traversable
    {
        return new \ArrayIterator([]);
    }

    /** @param TValue $value */
    public function take($value): void
    {
        var_dump($value);
    }

    public function each(): void
    {
        foreach ($this as $value) {
            $this->take($value);
        }
    }
}

/** @phpstan-assert-if-true array<mixed> $value */
function isArray(mixed $value): bool
{
    return is_array($value);
}

/**
 * An asserting call's `$a['k'] ?? null` argument specifies `$a['k']`.
 *
 * @param array<string, mixed> $options
 * @return array<mixed>
 */
function options(array $options): array
{
    return isArray($options['multipart'] ?? null) ? $options['multipart'] : [];
}

/** debug_backtrace() returns its functionMap frames. */
function caller(): string
{
    [, , $caller] = debug_backtrace(DEBUG_BACKTRACE_IGNORE_ARGS, 3);

    return $caller['function'];
}

/** array_key_exists() with a constant key proves an array either way. */
function driver(mixed $config): mixed
{
    if (array_key_exists('driver', $config)) {
        return $config['driver'];
    }

    return $config['fallback'] ?? null;
}

/** An undefined variable passed to a by-reference parameter that creates it is not checked. */
class Declaring
{
    protected function find(string $name, ?\ReflectionClass &$declaring = null): bool
    {
        $declaring = null;

        return $name !== '';
    }

    public function check(): bool
    {
        return $this->find('x', $declaring);
    }
}

/**
 * A closure argument infers nothing from `iterable<TKey, TValue>` and its templates from the Closure member.
 *
 * @template TKey of array-key
 * @template TValue
 */
class Generated
{
    /** @param iterable<TKey, TValue>|(\Closure(): \Generator<TKey, TValue, mixed, void>)|null $source */
    public function __construct($source = null)
    {
        var_dump($source);
    }
}

/** @param Generated<int, string> $generated */
function takesGenerated(Generated $generated): void
{
    var_dump($generated);
}

function generated(): void
{
    takesGenerated(new Generated(function () {
        yield 'a' => 1;
    }));
}

/** A yield in a nested closure makes the function a generator, whose returns are not checked against `int`. */
function nestedYield(): int
{
    $numbers = function () {
        yield 1;
    };
    var_dump($numbers);

    return 'one';
}

class Relation
{
    public function where(): static
    {
        return $this;
    }
}

/** A closure item of an array argument takes its parameter types from the parameter's array value type. */
class Eager
{
    /** @param array<array-key, (\Closure(Relation): mixed)|string>|string $relations */
    public function with($relations): void
    {
        var_dump($relations);
    }

    public function load(): void
    {
        $this->with(['posts' => static fn ($query) => $query->where()]);
        $this->with(['posts' => static function ($query): void {
            $query->missing();
        }]);
    }
}

/** A method call without a method reflection is an implicit throw point: the catch is analysed. */
function forward(mixed $object, string $method): mixed
{
    try {
        return $object->{$method}();
    } catch (\Error $e) {
        return get_class($object);
    }
}

/** str_replace() of a mixed subject returns a benevolent union. */
function replaced(mixed $subject): string
{
    return str_replace('a', '', $subject);
}

/**
 * An array plus a mixed is `array<mixed, mixed>`.
 *
 * @param array<int, string> $defaults
 */
function merged(array $defaults, mixed $options): int
{
    return $defaults + $options;
}

/**
 * A PHPDoc variadic of a plain parameter leaves the body's variable as declared.
 *
 * @param mixed ...$tags
 */
function tags($tags): string
{
    return $tags;
}

enum Grade
{
    case One;
    case Two;
    case Three;
}

class Flags
{
    public bool $flag = false;
}

/**
 * A conjunction's holder condition is its side's falsey narrowing: a disjunction side naming several expressions
 * builds no holder, so `$forced` does not prove `$flags->flag` later.
 */
function disjunctionSide(Grade $grade, Flags $flags, bool $extra, bool $cond): bool
{
    $forced = false;
    if ($grade !== Grade::Three && $flags->flag === true) {
        $forced = true;
    }
    if ($forced === false && (($grade === Grade::One && $extra === false) || ($cond && $grade !== Grade::Three))) {
        return false;
    }
    if ($grade !== Grade::Three) {
        return $flags->flag === false;
    }

    return true;
}

class Keys
{
    /**
     * @template TKey of string
     * @template TValue
     * @param array<TKey, TValue> $array
     * @return \Generator<TKey, TValue, void, void>
     */
    public static function stringify(array $array): \Generator
    {
        foreach ($array as $key => $value) {
            yield (string) $key => $value;
        }
    }

    /**
     * A benevolent `(int|string)` key infers the template's matching `string` member, so it resolves.
     */
    public function globals(): void
    {
        self::stringify($GLOBALS);
    }
}
