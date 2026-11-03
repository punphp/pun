<?php

namespace Laravel\Prompts\Themes\Default;

use Laravel\Prompts\Concerns\HasSpinner;
use Laravel\Prompts\Task;

class TaskRenderer extends Renderer
{
    use HasSpinner;

    /**
     * Render the task.
     */
    public function __invoke(Task $task): string
    {}

    protected function stableMessageSymbol(string $type): string
    {}
}
