<?php

namespace Laravel\Prompts\Concerns;

use Closure;

trait Events
{
    /**
     * The registered event listeners.
     *
     * @var array<string, array<int, Closure>>
     */
    protected array $listeners = [];

    /**
     * Register an event listener.
     */
    public function on(string $event, Closure $callback): void
    {}

    /**
     * Emit an event.
     */
    public function emit(string $event, mixed ...$data): void
    {}

    /**
     * Clean the event listeners.
     */
    public function clearListeners(): void
    {}
}
