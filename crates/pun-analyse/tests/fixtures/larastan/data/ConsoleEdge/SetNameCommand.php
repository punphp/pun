<?php

namespace App\Console\Commands\Pun;

use Illuminate\Console\Command;
use Symfony\Component\Console\Input\InputArgument;

class SetNameCommand extends Command
{
    protected $signature = 'pun:set-name';

    protected function configure(): void
    {
        $this->setName('pun:renamed');
    }

    public function handle(): void
    {
        $this->argument('command');
    }
}
