<?php

namespace App\Console\Commands\Pun;

use Illuminate\Console\Command;

abstract class BaseCommand extends Command
{
    protected function shared(): void
    {
        $this->argument('shared');
        $this->option('shared');
    }
}
