<?php

namespace App\Console\Commands\Pun;

use Illuminate\Console\Command;
use Symfony\Component\Console\Input\InputArgument;

class ConstNameCommand extends Command
{
    protected $name = 'pun:const-name';

    private const ARG = 'const-arg';

    protected function getArguments(): array
    {
        return [[self::ARG]];
    }

    public function handle(): void
    {
        $this->argument('const-arg');
    }
}
