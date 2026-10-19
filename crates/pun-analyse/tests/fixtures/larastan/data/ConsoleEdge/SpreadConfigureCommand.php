<?php

namespace App\Console\Commands\Pun;

use Illuminate\Console\Command;
use Symfony\Component\Console\Input\InputArgument;

class SpreadConfigureCommand extends Command
{
    protected $signature = 'pun:spread-configure';

    protected function configure(): void
    {
        $args = ['spread-arg'];
        $this->addArgument(...$args);
    }

    public function handle(): void
    {
        $this->argument('spread-arg');
    }
}
