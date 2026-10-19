<?php

namespace App\Console\Commands\Pun;

use Illuminate\Console\Command;
use Symfony\Component\Console\Input\InputArgument;

class DynamicNewCommand extends Command
{
    protected $name = 'pun:dynamic-new';

    protected function getArguments(): array
    {
        $class = InputArgument::class;

        return [new $class('dyn')];
    }

    public function handle(): void
    {
        $this->argument('dyn');
    }
}
