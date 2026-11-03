<?php

namespace Illuminate\Console;

use function Laravel\Prompts\confirm;

trait ConfirmableTrait
{
    /**
     * Confirm before proceeding with the action.
     *
     * This method only asks for confirmation in production.
     *
     * @template TReturn of bool = bool
     *
     * @param  string  $warning
     * @param  (\Closure(): TReturn)|TReturn|null  $callback
     * @return (TReturn is false ? true : bool)
     */
    public function confirmToProceed($warning = 'Application In Production', $callback = null)
    {}

    /**
     * Get the default confirmation callback.
     *
     * @return \Closure(): bool
     */
    protected function getDefaultConfirmCallback()
    {}
}
