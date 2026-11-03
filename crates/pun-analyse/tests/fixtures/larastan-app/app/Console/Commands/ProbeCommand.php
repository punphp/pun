<?php

namespace App\Console\Commands;

use Illuminate\Console\Command;

use function PHPStan\dumpType;

final class ProbeCommand extends Command
{
    protected $signature = 'probe {argument} {item?} {level=1} {list?*} {--F|force} {--limit=} {--tag=*}';

    public function handle(): void
    {
        dumpType($this->argument());
        dumpType($this->arguments());
        dumpType($this->argument('argument'));
        dumpType($this->argument('item'));
        dumpType($this->argument('level'));
        dumpType($this->argument('list'));
        dumpType($this->option('F'));
        dumpType($this->option('force'));
        dumpType($this->option('limit'));
        dumpType($this->option('tag'));
        dumpType($this->option('ansi'));
        dumpType($this->option('v'));
        dumpType($this->hasArgument('item'));
        dumpType($this->hasArgument('missing'));
        dumpType($this->hasOption('v'));
        dumpType($this->hasOption('missing'));
    }
}
