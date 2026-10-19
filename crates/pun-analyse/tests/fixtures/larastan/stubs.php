<?php

// The Laravel 12 declarations (laravel/framework, as Larastan 3.10.0's test application installs it) that
// tests/larastan.rs's fixtures use, cut down to the members the Larastan rules read.

namespace Illuminate\Contracts\Container {
    interface Container
    {
        public function bind($abstract, $concrete = null, $shared = false);

        public function singleton($abstract, $concrete = null);
    }
}

namespace Illuminate\Contracts\Foundation {
    interface Application extends \Illuminate\Contracts\Container\Container
    {
    }
}

namespace Illuminate\Container {
    class Container implements \Illuminate\Contracts\Container\Container
    {
        public function bind($abstract, $concrete = null, $shared = false)
        {
        }

        public function singleton($abstract, $concrete = null)
        {
        }
    }
}

namespace Illuminate\Foundation {
    class Application extends \Illuminate\Container\Container implements \Illuminate\Contracts\Foundation\Application
    {
    }
}

namespace Illuminate\Contracts\Support {
    interface DeferrableProvider
    {
    }
}

namespace Illuminate\Support {
    abstract class ServiceProvider
    {
        /**
         * The application instance.
         *
         * @var \Illuminate\Contracts\Foundation\Application
         */
        protected $app;

        /**
         * @return array
         */
        public function provides()
        {
            return [];
        }
    }
}

namespace Illuminate\Support\Facades {
    abstract class Facade
    {
    }

    class Schema extends Facade
    {
    }

    class Auth extends Facade
    {
    }

    class View extends Facade
    {
    }

    class Route extends Facade
    {
    }

    class Lang extends Facade
    {
    }
}

namespace Illuminate\Contracts\View {
    interface View
    {
    }

    interface Factory
    {
        /** @return \Illuminate\Contracts\View\View */
        public function make($view, $data = [], $mergeData = []);
    }
}

namespace Illuminate\View {
    class Factory implements \Illuminate\Contracts\View\Factory
    {
        public function make($view, $data = [], $mergeData = [])
        {
        }
    }
}

namespace Illuminate\Mail {
    class Mailable
    {
        /** @return $this */
        public function markdown($view, array $data = [])
        {
            return $this;
        }

        /** @return $this */
        public function view($view, array $data = [])
        {
            return $this;
        }
    }
}

namespace Illuminate\Notifications\Messages {
    class MailMessage
    {
        /** @return $this */
        public function markdown($view, array $data = [])
        {
            return $this;
        }

        /** @return $this */
        public function view($view, array $data = [])
        {
            return $this;
        }
    }
}

namespace Illuminate\Translation {
    class Translator
    {
        public function get($key, array $replace = [], $locale = null, $fallback = true)
        {
        }

        public function choice($key, $number, array $replace = [], $locale = null)
        {
        }
    }
}

namespace Illuminate\Contracts\Auth {
    interface Factory
    {
    }

    interface Guard
    {
    }
}

namespace Illuminate\Auth {
    class AuthManager implements \Illuminate\Contracts\Auth\Factory
    {
        public function __call($method, $parameters)
        {
        }
    }
}

namespace Illuminate\Http {
    class Request
    {
        public function user($guard = null)
        {
        }
    }
}

namespace Illuminate\Contracts\Support {
    /**
     * @template TKey of array-key
     * @template TValue
     */
    interface Arrayable
    {
        /** @return array<TKey, TValue> */
        public function toArray();
    }
}

namespace Illuminate\Support {
    /**
     * @template TKey of array-key
     * @template-covariant TValue
     * @extends \Illuminate\Contracts\Support\Arrayable<TKey, TValue>
     * @extends \IteratorAggregate<TKey, TValue>
     */
    interface Enumerable extends \Illuminate\Contracts\Support\Arrayable, \Countable, \IteratorAggregate
    {
    }

    /**
     * @template TKey of array-key
     * @template-covariant TValue
     * @implements \Illuminate\Support\Enumerable<TKey, TValue>
     */
    class Collection implements Enumerable
    {
        /** @return array<TKey, mixed> */
        public function toArray()
        {
        }

        /** @return array<TKey, TValue> */
        public function all()
        {
        }

        public function count(): int
        {
        }

        /** @return static */
        public function map(callable $callback)
        {
        }

        public function getIterator(): \ArrayIterator
        {
        }
    }

    /**
     * @template TKey of array-key
     * @template-covariant TValue
     * @implements \Illuminate\Support\Enumerable<TKey, TValue>
     */
    class LazyCollection implements Enumerable
    {
        /** @return array<TKey, mixed> */
        public function toArray()
        {
        }

        public function count(): int
        {
        }

        public function getIterator(): \Traversable
        {
        }
    }
}

namespace Illuminate\Database\Eloquent {
    /**
     * @template TKey of array-key
     * @template TModel of \Illuminate\Database\Eloquent\Model
     * @extends \Illuminate\Support\Collection<TKey, TModel>
     */
    class Collection extends \Illuminate\Support\Collection
    {
    }

    abstract class Model implements \Illuminate\Contracts\Support\Arrayable
    {
        /** @return \Illuminate\Database\Eloquent\Collection<int, static> */
        public static function all($columns = ['*'])
        {
        }

        /** @return \Illuminate\Database\Eloquent\Builder<static> */
        public static function query()
        {
        }

        /**
         * @param  array|string  $relations
         * @return \Illuminate\Database\Eloquent\Builder<static>
         */
        public static function with($relations)
        {
        }

        /**
         * @template TRelatedModel of Model
         * @param  class-string<TRelatedModel>  $related
         * @return \Illuminate\Database\Eloquent\Relations\HasMany<TRelatedModel, $this>
         */
        public function hasMany($related)
        {
        }

        /**
         * @template TRelatedModel of Model
         * @param  class-string<TRelatedModel>  $related
         * @return \Illuminate\Database\Eloquent\Relations\HasOne<TRelatedModel, $this>
         */
        public function hasOne($related)
        {
        }

        /**
         * @template TRelatedModel of Model
         * @param  class-string<TRelatedModel>  $related
         * @return \Illuminate\Database\Eloquent\Relations\BelongsTo<TRelatedModel, $this>
         */
        public function belongsTo($related)
        {
        }

        public function toArray()
        {
        }

        public static function __callStatic($method, $parameters)
        {
        }

        public function __call($method, $parameters)
        {
        }
    }

    /**
     * @template TModel of \Illuminate\Database\Eloquent\Model
     */
    class Builder
    {
        public function where($column, $operator = null, $value = null, $boolean = 'and')
        {
            return $this;
        }

        /** @return \Illuminate\Database\Eloquent\Collection<int, TModel> */
        public function get($columns = ['*'])
        {
        }

        /** @return \Illuminate\Support\Collection<array-key, mixed> */
        public function pluck($column, $key = null)
        {
        }

        /** @return $this */
        public function has($relation)
        {
        }

        /** @return $this */
        public function orHas($relation)
        {
        }

        /** @return $this */
        public function doesntHave($relation)
        {
        }

        /** @return $this */
        public function orDoesntHave($relation)
        {
        }

        /** @return $this */
        public function whereHas($relation)
        {
        }

        /** @return $this */
        public function withWhereHas($relation)
        {
        }

        /** @return $this */
        public function orWhereHas($relation)
        {
        }

        /** @return $this */
        public function whereDoesntHave($relation)
        {
        }

        /** @return $this */
        public function orWhereDoesntHave($relation)
        {
        }

        /** @return $this */
        public function whereRelation($relation)
        {
        }

        /** @return $this */
        public function with($relation)
        {
        }
    }
}

namespace Illuminate\Database\Eloquent\Relations {
    /**
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
     * @template TResult
     * @mixin \Illuminate\Database\Eloquent\Builder<TRelatedModel>
     */
    abstract class Relation
    {
        /** @return \Illuminate\Database\Eloquent\Collection<int, TRelatedModel> */
        public function get($columns = ['*'])
        {
        }

        /** @return $this */
        public function has($relation)
        {
        }

        /** @return $this */
        public function orHas($relation)
        {
        }

        /** @return $this */
        public function doesntHave($relation)
        {
        }

        /** @return $this */
        public function orDoesntHave($relation)
        {
        }

        /** @return $this */
        public function whereHas($relation)
        {
        }

        /** @return $this */
        public function withWhereHas($relation)
        {
        }

        /** @return $this */
        public function orWhereHas($relation)
        {
        }

        /** @return $this */
        public function whereDoesntHave($relation)
        {
        }

        /** @return $this */
        public function orWhereDoesntHave($relation)
        {
        }

        /** @return $this */
        public function whereRelation($relation)
        {
        }

        /** @return $this */
        public function with($relation)
        {
        }
    }

    /**
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
     * @extends Relation<TRelatedModel, TDeclaringModel, \Illuminate\Database\Eloquent\Collection<int, TRelatedModel>>
     */
    class HasMany extends Relation
    {
    }

    /**
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
     * @extends Relation<TRelatedModel, TDeclaringModel, ?TRelatedModel>
     */
    class HasOne extends Relation
    {
    }

    /**
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
     * @extends Relation<TRelatedModel, TDeclaringModel, ?TRelatedModel>
     */
    class BelongsTo extends Relation
    {
    }

    /**
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
     * @extends Relation<TRelatedModel, TDeclaringModel, \Illuminate\Database\Eloquent\Collection<int, TRelatedModel>>
     */
    class BelongsToMany extends Relation
    {
    }

    /**
     * @template TRelatedModel of \Illuminate\Database\Eloquent\Model
     * @template TIntermediateModel of \Illuminate\Database\Eloquent\Model
     * @template TDeclaringModel of \Illuminate\Database\Eloquent\Model
     * @extends Relation<TRelatedModel, TIntermediateModel, \Illuminate\Database\Eloquent\Collection<int, TRelatedModel>>
     */
    class HasManyThrough extends Relation
    {
    }
}

namespace Illuminate\Database\Eloquent\Casts {
    class Attribute
    {
        public static function make(?callable $get = null, ?callable $set = null): static
        {
            return new static();
        }
    }
}

namespace Illuminate\Database\Eloquent\Attributes {
    #[\Attribute(\Attribute::TARGET_METHOD)]
    class Scope
    {
    }
}

namespace Illuminate\Foundation\Bus {
    trait Dispatchable
    {
        public static function dispatch(...$arguments)
        {
        }

        public static function dispatchIf($boolean, ...$arguments)
        {
        }

        public static function dispatchUnless($boolean, ...$arguments)
        {
        }

        public static function dispatchSync(...$arguments)
        {
        }

        public static function dispatchAfterResponse(...$arguments)
        {
        }
    }
}

namespace Illuminate\Foundation\Events {
    trait Dispatchable
    {
        public static function dispatch(...$arguments)
        {
        }

        public static function dispatchIf($boolean, ...$arguments)
        {
        }

        public static function dispatchUnless($boolean, ...$arguments)
        {
        }
    }
}

namespace App {
    use Illuminate\Database\Eloquent\Relations\BelongsTo;
    use Illuminate\Database\Eloquent\Relations\BelongsToMany;
    use Illuminate\Database\Eloquent\Relations\HasMany;
    use Illuminate\Database\Eloquent\Relations\HasManyThrough;

    // the relations of Larastan's test application's models
    class User extends \Illuminate\Database\Eloquent\Model
    {
        /** @phpstan-return BelongsTo<Group, $this> */
        public function group(): BelongsTo
        {
        }

        /** @phpstan-return HasMany<Account, $this> */
        public function accounts(): HasMany
        {
        }

        /** @return HasManyThrough<Transaction, Account, $this> */
        public function transactions(): HasManyThrough
        {
        }
    }

    class Group extends \Illuminate\Database\Eloquent\Model
    {
        /** @phpstan-return HasMany<Account, $this> */
        public function accounts(): HasMany
        {
        }

        /** @phpstan-return HasMany<User, $this> */
        public function users(): HasMany
        {
        }
    }

    class Account extends \Illuminate\Database\Eloquent\Model
    {
        /** @return HasMany<Transaction, $this> */
        public function transactions(): HasMany
        {
        }

        /** @phpstan-return BelongsTo<Group, $this> */
        public function group(): BelongsTo
        {
        }
    }

    class Transaction extends \Illuminate\Database\Eloquent\Model
    {
    }

    class Post extends \Illuminate\Database\Eloquent\Model
    {
        /** @return BelongsToMany<User, $this> */
        public function users(): BelongsToMany
        {
        }
    }
}

namespace {
    function with($value, ?callable $callback = null)
    {
        return is_null($callback) ? $value : $callback($value);
    }

    function value($value, ...$args)
    {
        return $value instanceof Closure ? $value(...$args) : $value;
    }

    /**
     * @template TKey of array-key
     * @template TValue
     * @param  array<TKey, TValue>  $value
     * @return \Illuminate\Support\Collection<TKey, TValue>
     */
    function collect($value = [])
    {
    }

    /**
     * @return ($view is null ? \Illuminate\Contracts\View\Factory : \Illuminate\Contracts\View\View)
     */
    function view($view = null, $data = [], $mergeData = [])
    {
    }

    function __($key = null, $replace = [], $locale = null)
    {
    }

    function trans($key = null, $replace = [], $locale = null)
    {
    }

    function trans_choice($key, $number, array $replace = [], $locale = null)
    {
    }

    function env($key, $default = null)
    {
    }

    /**
     * @param  string  $path
     * @return string
     */
    function storage_path($path = '')
    {
    }

    /**
     * @param  string|null  $guard
     * @return ($guard is null ? \Illuminate\Contracts\Auth\Factory : \Illuminate\Contracts\Auth\Guard)
     */
    function auth($guard = null): \Illuminate\Contracts\Auth\Factory|\Illuminate\Contracts\Auth\Guard
    {
    }
}
