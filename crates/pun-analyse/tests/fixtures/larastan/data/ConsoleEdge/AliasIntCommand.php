<?php

namespace App\Console\Commands\Pun;

use Illuminate\Console\Command;

class AliasIntCommand extends Command
{
    protected $signature = 'pun:alias-int';

    protected $aliases = 5;

    public function handle(): void
    {
        $this->argument('command');
    }
}
