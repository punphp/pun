<?php

namespace Illuminate\Log\Context;

use __PHP_Incomplete_Class;
use Closure;
use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Database\Eloquent\ModelNotFoundException;
use Illuminate\Log\Context\Events\ContextDehydrating as Dehydrating;
use Illuminate\Log\Context\Events\ContextHydrated as Hydrated;
use Illuminate\Queue\SerializesModels;
use Illuminate\Support\Collection;
use Illuminate\Support\Traits\Conditionable;
use Illuminate\Support\Traits\Macroable;
use RuntimeException;
use Throwable;

class Repository
{
    use Conditionable, Macroable, SerializesModels;

    /**
     * The event dispatcher instance.
     *
     * @var \Illuminate\Contracts\Events\Dispatcher
     */
    protected $events;

    /**
     * The contextual data.
     *
     * @var array<string, mixed>
     */
    protected $data = [];

    /**
     * The hidden contextual data.
     *
     * @var array<string, mixed>
     */
    protected $hidden = [];

    /**
     * The callback that should handle unserialize exceptions.
     *
     * @var callable|null
     */
    protected static $handleUnserializeExceptionsUsing;

    /**
     * Create a new Context instance.
     */
    public function __construct(Dispatcher $events)
    {}

    /**
     * Determine if the given key exists.
     *
     * @param  string  $key
     * @return bool
     */
    public function has($key)
    {}

    /**
     * Determine if the given key is missing.
     *
     * @param  string  $key
     * @return bool
     */
    public function missing($key)
    {}

    /**
     * Determine if the given key exists within the hidden context data.
     *
     * @param  string  $key
     * @return bool
     */
    public function hasHidden($key)
    {}

    /**
     * Determine if the given key is missing within the hidden context data.
     *
     * @param  string  $key
     * @return bool
     */
    public function missingHidden($key)
    {}

    /**
     * Retrieve all the context data.
     *
     * @return array<string, mixed>
     */
    public function all()
    {}

    /**
     * Retrieve all the hidden context data.
     *
     * @return array<string, mixed>
     */
    public function allHidden()
    {}

    /**
     * Retrieve the given key's value.
     *
     * @param  string  $key
     * @param  mixed  $default
     * @return mixed
     */
    public function get($key, $default = null)
    {}

    /**
     * Retrieve the given key's hidden value.
     *
     * @param  string  $key
     * @param  mixed  $default
     * @return mixed
     */
    public function getHidden($key, $default = null)
    {}

    /**
     * Retrieve the given key's value and then forget it.
     *
     * @param  string  $key
     * @param  mixed  $default
     * @return mixed
     */
    public function pull($key, $default = null)
    {}

    /**
     * Retrieve the given key's hidden value and then forget it.
     *
     * @param  string  $key
     * @param  mixed  $default
     * @return mixed
     */
    public function pullHidden($key, $default = null)
    {}

    /**
     * Retrieve only the values of the given keys.
     *
     * @param  array<int, string>  $keys
     * @return array<string, mixed>
     */
    public function only($keys)
    {}

    /**
     * Retrieve only the hidden values of the given keys.
     *
     * @param  array<int, string>  $keys
     * @return array<string, mixed>
     */
    public function onlyHidden($keys)
    {}

    /**
     * Retrieve all values except those with the given keys.
     *
     * @param  array<int, string>  $keys
     * @return array<string, mixed>
     */
    public function except($keys)
    {}

    /**
     * Retrieve all hidden values except those with the given keys.
     *
     * @param  array<int, string>  $keys
     * @return array<string, mixed>
     */
    public function exceptHidden($keys)
    {}

    /**
     * Add a context value.
     *
     * @param  string|array<string, mixed>  $key
     * @param  mixed  $value
     * @return $this
     */
    public function add($key, $value = null)
    {}

    /**
     * Add a hidden context value.
     *
     * @param  string|array<string, mixed>  $key
     * @param  mixed  $value
     * @return $this
     */
    public function addHidden($key, #[\SensitiveParameter] $value = null)
    {}

    /**
     * Add a context value if it does not exist yet, and return the value.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return mixed
     */
    public function remember($key, $value)
    {}

    /**
     * Add a hidden context value if it does not exist yet, and return the value.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return mixed
     */
    public function rememberHidden($key, #[\SensitiveParameter] $value)
    {}

    /**
     * Forget the given context key.
     *
     * @param  string|array<int, string>  $key
     * @return $this
     */
    public function forget($key)
    {}

    /**
     * Forget the given hidden context key.
     *
     * @param  string|array<int, string>  $key
     * @return $this
     */
    public function forgetHidden($key)
    {}

    /**
     * Add a context value if it does not exist yet.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return $this
     */
    public function addIf($key, $value)
    {}

    /**
     * Add a hidden context value if it does not exist yet.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return $this
     */
    public function addHiddenIf($key, #[\SensitiveParameter] $value)
    {}

    /**
     * Push the given values onto the key's stack.
     *
     * @param  string  $key
     * @param  mixed  ...$values
     * @return $this
     *
     * @throws \RuntimeException
     */
    public function push($key, ...$values)
    {}

    /**
     * Pop the latest value from the key's stack.
     *
     * @param  string  $key
     * @return mixed
     *
     * @throws \RuntimeException
     */
    public function pop($key)
    {}

    /**
     * Push the given hidden values onto the key's stack.
     *
     * @param  string  $key
     * @param  mixed  ...$values
     * @return $this
     *
     * @throws \RuntimeException
     */
    public function pushHidden($key, ...$values)
    {}

    /**
     * Pop the latest hidden value from the key's stack.
     *
     * @param  string  $key
     * @return mixed
     *
     * @throws \RuntimeException
     */
    public function popHidden($key)
    {}

    /**
     * Increment a context counter.
     *
     * @param  string  $key
     * @param  int  $amount
     * @return $this
     */
    public function increment(string $key, int $amount = 1)
    {}

    /**
     * Decrement a context counter.
     *
     * @param  string  $key
     * @param  int  $amount
     * @return $this
     */
    public function decrement(string $key, int $amount = 1)
    {}

    /**
     * Determine if the given value is in the given stack.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @param  bool  $strict
     * @return bool
     *
     * @throws \RuntimeException
     */
    public function stackContains(string $key, mixed $value, bool $strict = false): bool
    {}

    /**
     * Determine if the given value is in the given hidden stack.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @param  bool  $strict
     * @return bool
     *
     * @throws \RuntimeException
     */
    public function hiddenStackContains(string $key, mixed $value, bool $strict = false): bool
    {}

    /**
     * Determine if a given key can be used as a stack.
     *
     * @param  string  $key
     * @return bool
     */
    protected function isStackable($key)
    {}

    /**
     * Determine if a given key can be used as a hidden stack.
     *
     * @param  string  $key
     * @return bool
     */
    protected function isHiddenStackable($key)
    {}

    /**
     * @template TReturn of mixed
     *
     * Run the callback function with the given context values and restore the original context state when complete.
     *
     * @param  (callable(): TReturn)  $callback
     * @param  array<string, mixed>  $data
     * @param  array<string, mixed>  $hidden
     * @return TReturn
     *
     * @throws \Throwable
     */
    public function scope(callable $callback, array $data = [], array $hidden = [])
    {}

    /**
     * Determine if the repository is empty.
     *
     * @return bool
     */
    public function isEmpty()
    {}

    /**
     * Execute the given callback when context is about to be dehydrated.
     *
     * @param  (callable(static): void)  $callback
     * @return $this
     */
    public function dehydrating($callback)
    {}

    /**
     * Execute the given callback when context has been hydrated.
     *
     * @param  (callable(static): void)  $callback
     * @return $this
     */
    public function hydrated($callback)
    {}

    /**
     * Handle unserialize exceptions using the given callback.
     *
     * @param  callable|null  $callback
     * @return $this
     */
    public function handleUnserializeExceptionsUsing($callback)
    {}

    /**
     * Flush all context data.
     *
     * @return $this
     */
    public function flush()
    {}

    /**
     * Dehydrate the context data.
     *
     * @internal
     *
     * @return ?array
     */
    public function dehydrate()
    {}

    /**
     * Hydrate the context instance.
     *
     * @internal
     *
     * @param  ?array  $context
     * @return $this
     *
     * @throws \RuntimeException
     */
    public function hydrate($context)
    {}
}
