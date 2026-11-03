<?php

namespace Illuminate\Console\Concerns;

use Illuminate\Support\Collection;
use Symfony\Component\Finder\Finder;

trait FindsAvailableModels
{
    /**
     * Get a list of possible model names.
     *
     * @return array<int, string>
     */
    protected function findAvailableModels()
    {}
}
