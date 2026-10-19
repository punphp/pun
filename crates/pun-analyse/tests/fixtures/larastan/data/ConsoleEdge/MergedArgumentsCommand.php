<?php

namespace App\Console\Commands\Pun;

use Illuminate\Console\Command;

class MergedArgumentsCommand extends Command
{
    protected $name = 'pun:merged-arguments';

    protected function getArguments(): array
    {
        return array_merge([['merged']], []);
    }

    public function handle(): void
    {
        $this->argument('merged');
    }
}
