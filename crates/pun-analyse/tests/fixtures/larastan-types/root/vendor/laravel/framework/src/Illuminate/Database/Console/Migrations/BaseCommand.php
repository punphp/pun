<?php

namespace Illuminate\Database\Console\Migrations;

use Illuminate\Console\Command;
use Illuminate\Support\Collection;

class BaseCommand extends Command
{
    /**
     * Get all of the migration paths.
     *
     * @return string[]
     */
    protected function getMigrationPaths()
    {}

    /**
     * Determine if the given path(s) are pre-resolved "real" paths.
     *
     * @return bool
     */
    protected function usingRealPath()
    {}

    /**
     * Get the path to the migration directory.
     *
     * @return string
     */
    protected function getMigrationPath()
    {}
}
