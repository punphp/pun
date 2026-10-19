<?php

namespace App\Console\Commands\Pun;

class ChildTwoCommand extends BaseCommand
{
    protected $signature = 'pun:child-two {--shared}';

    public function handle(): void
    {
        $this->shared();
    }
}
