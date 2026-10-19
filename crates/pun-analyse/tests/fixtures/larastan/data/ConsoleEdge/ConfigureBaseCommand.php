<?php

namespace App\Console\Commands\Pun;

use Illuminate\Console\Command;

abstract class ConfigureBaseCommand extends Command
{
    protected function configure(): void
    {
        $this->addArgument('base-arg');
    }
}
