<?php

namespace App\Console\Commands\Pun;

use Illuminate\Console\Attributes\Signature;
use Illuminate\Console\Command;

#[Signature('pun:sig-attr {arg} {--opt}', aliases: ['pun:sa'])]
class SignatureAttrCommand extends Command
{
    public function handle(): void
    {
        $this->argument('arg');
        $this->argument('nope');
        $this->option('opt');
    }
}
