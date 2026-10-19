<?php

namespace App\Console\Commands\Pun;

use Illuminate\Console\Command;
use Illuminate\Contracts\Console\Isolatable;
use Symfony\Component\Console\Input\InputArgument;
use Symfony\Component\Console\Input\InputOption;

class ParametersCommand extends Command implements Isolatable
{
    protected $name = 'pun:parameters';

    protected $aliases = ['pun:params'];

    protected function configure(): void
    {
        $this->addArgument('configured')
            ->addOption('configured-option', 'c');
    }

    protected function getArguments(): array
    {
        return [
            ['first', InputArgument::OPTIONAL, 'The first'],
            new InputArgument('second', InputArgument::OPTIONAL),
        ];
    }

    protected function getOptions(): array
    {
        return [
            ['force', 'f', InputOption::VALUE_NONE, 'Force'],
            new InputOption('limit', '-l|L', InputOption::VALUE_REQUIRED),
        ];
    }

    public function handle(): void
    {
        $this->argument('first');
        $this->argument('second');
        $this->argument('configured');
        $this->argument('third');
        $this->option('force');
        $this->option('f');
        $this->option('limit');
        $this->option('l');
        $this->option('L');
        $this->option('c');
        $this->option('configured-option');
        $this->option('isolated');
        $this->option('missing');
    }
}
