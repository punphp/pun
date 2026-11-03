<?php

namespace Illuminate\Console\View\Components;

use Symfony\Component\Console\Output\OutputInterface;

class TwoColumnDetail extends Component
{
    /**
     * Renders the component using the given arguments.
     *
     * @param  string  $first
     * @param  string|null  $second
     * @param  int  $verbosity
     * @return void
     */
    public function render($first, $second = null, $verbosity = OutputInterface::VERBOSITY_NORMAL)
    {}
}
