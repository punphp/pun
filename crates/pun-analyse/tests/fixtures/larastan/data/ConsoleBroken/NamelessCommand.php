<?php

namespace App\Console\Commands\Pun;

use Illuminate\Console\Command;

class NamelessCommand extends Command
{
    public function handle(): void
    {
        $this->option('nope');
    }
}
