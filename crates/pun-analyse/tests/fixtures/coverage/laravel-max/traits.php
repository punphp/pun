<?php

namespace LaravelMax\Traits;

interface Application
{
}

trait Queueable
{
    /** @var string|null */
    public $connection;

    public function onConnection(mixed $connection): void
    {
        $this->connection = $connection;
    }
}

trait CreatesProviders
{
    public function provider(): int
    {
        return $this->app;
    }

    public function constants(): void
    {
        $this->takesInt(self::CREATED_AT);
        $this->takesInt(static::make());
        $this->takesInt(static::$items);
    }

    public function className(): void
    {
        $this->takesInt(get_class($this));
    }
}

class Manager
{
    use CreatesProviders;
    use Queueable;

    const CREATED_AT = 'created_at';

    /** @var list<int> */
    public static array $items = [];

    /** @var Application */
    protected $app;

    public function __construct(Application $app)
    {
        $this->app = $app;
    }

    public static function make(): string
    {
        return '';
    }

    public function takesInt(int $int): void
    {
    }
}

/**
 * @template TKey of array-key
 * @template-covariant TValue
 */
trait EnumeratesValues
{
    /**
     * @template TMakeKey of array-key
     * @template TMakeValue
     * @param  iterable<TMakeKey, TMakeValue>|null  $items
     * @return static<TMakeKey, TMakeValue>
     */
    public static function make($items = [])
    {
        return new static($items);
    }
}

/**
 * @template TKey of array-key
 * @template-covariant TValue
 */
final class Collection
{
    /** @use EnumeratesValues<TKey, TValue> */
    use EnumeratesValues;

    /** @param iterable<TKey, TValue>|null $items */
    public function __construct($items = [])
    {
        foreach ($items ?? [] as $item) {
        }
    }
}
