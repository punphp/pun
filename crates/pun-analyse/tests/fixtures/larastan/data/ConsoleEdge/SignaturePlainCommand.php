<?php

namespace App\Console\Commands\Pun;

use Illuminate\Console\Attributes\Signature;
use Illuminate\Console\Command;

#[Signature('pun:sig-plain {arg}')]
class SignaturePlainCommand extends Command
{
    protected $aliases = 'pun:sp';

    public function handle(): void
    {
        $this->argument('arg');
        $this->argument('nope');
    }
}

function pun_console_helper(Command $command): void
{
    $command->argument('nope');
}
