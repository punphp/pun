<?php

namespace App\Console\Commands\Pun;

use Illuminate\Console\Command;
use Symfony\Component\Console\Input\InputArgument;

class SpreadInnerCommand extends Command
{
    protected $name = 'pun:spread-inner';

    protected function getArguments(): array
    {
        $more = ['inner'];

        return [[...$more]];
    }

    public function handle(): void
    {
        $this->argument('inner');
    }
}
