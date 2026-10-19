<?php

namespace App\Console\Commands\Pun;

use Illuminate\Console\Command;

class KeyedAliasesCommand extends Command
{
    protected $signature = 'pun:keyed';

    protected $aliases = ['x' => 'pun:keyed-alias'];

    public function handle(): void
    {
        $this->argument('nope');
    }
}
