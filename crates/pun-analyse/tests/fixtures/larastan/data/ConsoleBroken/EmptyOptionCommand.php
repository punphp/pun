<?php

namespace App\Console\Commands\Pun;

use Illuminate\Console\Command;

class EmptyOptionCommand extends Command
{
    protected $signature = 'pun:empty-option {--}';

    public function handle(): void
    {
        $this->option('nope');
    }
}
