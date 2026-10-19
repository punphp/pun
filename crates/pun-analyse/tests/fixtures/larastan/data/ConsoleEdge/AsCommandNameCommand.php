<?php

namespace App\Console\Commands\Pun;

use Illuminate\Console\Command;
use Symfony\Component\Console\Attribute\AsCommand;
use Symfony\Component\Console\Input\InputArgument;
use Symfony\Component\Console\Input\InputOption;

#[AsCommand(name: 'pun:as-name')]
class AsCommandNameCommand extends Command
{
    protected function getArguments(): array
    {
        return array(
            array('legacy', InputArgument::OPTIONAL),
            ['name' => 'named', 'mode' => InputArgument::OPTIONAL],
            new InputArgument(name: 'named-arg', mode: InputArgument::OPTIONAL),
        );
    }

    protected function getOptions(): array
    {
        $unused = 1;

        return [
            ['opt', null],
            ['opt2', false],
            ['opt3', ['a', 'b']],
            new InputOption('opt4'),
        ];
    }

    public function handle(): void
    {
        $this->argument('legacy');
        $this->argument('named');
        $this->argument('named-arg');
        $this->argument('nope');
        $this->option('opt');
        $this->option('opt2');
        $this->option('opt3');
        $this->option('opt4');
        $this->option('a');
        $this->option('b');
        $this->option('nope');
    }
}
