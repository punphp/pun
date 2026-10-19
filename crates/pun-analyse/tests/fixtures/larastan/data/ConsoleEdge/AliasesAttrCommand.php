<?php

namespace App\Console\Commands\Pun;

use Illuminate\Console\Attributes\Aliases;
use Illuminate\Console\Command;

#[Aliases(['pun:aa'])]
class AliasesAttrCommand extends Command
{
    protected $signature = 'pun:aliases-attr';

    protected $aliases = null;

    public function handle(): void
    {
        $this->argument('nope');
    }
}
