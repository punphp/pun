<?php

namespace App\Console\Commands\Pun;

class ChildOneCommand extends BaseCommand
{
    protected $signature = 'pun:child-one {shared}';

    public function handle(): void
    {
        $this->shared();
        $this->option('shared');
    }
}
