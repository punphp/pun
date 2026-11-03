<?php

namespace Illuminate\Console\View\Components;

use Illuminate\Console\View\TaskResult;
use Illuminate\Support\InteractsWithTime;
use Symfony\Component\Console\Output\OutputInterface;
use Throwable;

use function Termwind\terminal;

class Task extends Component
{
    use InteractsWithTime;

    /**
     * Renders the component using the given arguments.
     *
     * @param  string  $description
     * @param  (callable(): bool)|null  $task
     * @param  int  $verbosity
     * @return void
     *
     * @throws \Throwable
     */
    public function render($description, $task = null, $verbosity = OutputInterface::VERBOSITY_NORMAL)
    {}
}
