<?php

namespace App\Console\Commands\Pun;

use Illuminate\Console\Command;
use Symfony\Component\Console\Input\InputArgument;

class KeyValueOuterCommand extends Command
{
    protected $name = 'pun:key-value-outer';

    protected function getArguments(): array
    {
        return ['k' => ['kv']];
    }

    public function handle(): void
    {
        $this->argument('kv');
    }
}
