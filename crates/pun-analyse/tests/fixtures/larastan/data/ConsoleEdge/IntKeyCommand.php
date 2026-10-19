<?php

namespace App\Console\Commands\Pun;

use Illuminate\Console\Command;
use Symfony\Component\Console\Input\InputArgument;

class IntKeyCommand extends Command
{
    protected $name = 'pun:int-key';

    protected function getArguments(): array
    {
        return [[0 => 'intkey']];
    }

    public function handle(): void
    {
        $this->argument('intkey');
    }
}
