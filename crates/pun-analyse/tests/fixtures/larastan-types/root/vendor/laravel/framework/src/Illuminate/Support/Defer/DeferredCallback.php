<?php

namespace Illuminate\Support\Defer;

use Illuminate\Support\Str;

class DeferredCallback
{
    /**
     * Create a new deferred callback instance.
     *
     * @param  callable  $callback
     */
    public function __construct(public $callback, public ?string $name = null, public bool $always = false)
    {}

    /**
     * Specify the name of the deferred callback so it can be cancelled later.
     *
     * @param  string  $name
     * @return $this
     */
    public function name(string $name): static
    {}

    /**
     * Indicate that the deferred callback should run even on unsuccessful requests and jobs.
     *
     * @param  bool  $always
     * @return $this
     */
    public function always(bool $always = true): static
    {}

    /**
     * Invoke the deferred callback.
     *
     * @return void
     */
    public function __invoke(): void
    {}
}
