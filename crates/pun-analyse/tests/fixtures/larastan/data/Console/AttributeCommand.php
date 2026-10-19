<?php

namespace App\Console\Commands\Pun;

use Illuminate\Console\Command;
use Symfony\Component\Console\Attribute\AsCommand;

#[AsCommand(name: 'pun:attribute')]
class AttributeCommand extends Command
{
    protected $signature = 'pun:attribute {known}';

    public function handle(): void
    {
        $this->argument('known');
        $this->argument('unknown');
    }
}
