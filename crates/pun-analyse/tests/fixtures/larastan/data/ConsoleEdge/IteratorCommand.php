<?php

namespace App\Console\Commands\Pun;

use Illuminate\Console\Command;
use Symfony\Component\Console\Input\InputArgument;

class IteratorCommand extends Command
{
    protected $name = 'pun:iterator';

    protected function getArguments(): array
    {
        return [new \ArrayIterator(['it'])];
    }

    public function handle(): void
    {
        $this->argument('it');
    }
}
