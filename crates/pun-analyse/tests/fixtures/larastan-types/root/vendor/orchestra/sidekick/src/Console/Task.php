<?php

namespace Orchestra\Sidekick\Console;

use Closure;

class Task
{
    /**
     * Construct a new pending task.
     *
     * @param  \Closure():(bool)  $action
     * @param  (\Closure(bool, bool):(void))|null  $response
     * @param  (\Closure():(bool))|bool  $requirement
     */
    public function __construct(
        protected Closure $action,
        protected ?Closure $response = null,
        protected Closure|bool $requirement = true,
    ) {}

    /**
     * Make a new pending task instance.
     *
     * @param  \Closure():(bool)  $action
     * @return static
     */
    public static function action(Closure $action)
    {}

    /**
     * Set the response callback for the task.
     *
     * @param  \Closure(bool, bool):(void)  $response
     * @return $this
     */
    public function response(Closure $response)
    {}

    /**
     * Set the requirement for the task.
     *
     * @param  (\Closure():(bool))|bool  $requirement
     * @return $this
     */
    public function requirements(Closure|bool $requirement)
    {}

    /**
     * Handle the task.
     */
    public function dispatch(bool $pretending = false): void
    {}

    /**
     * Handle the task when invoked.
     */
    public function __invoke(bool $pretending = false): void
    {}
}
