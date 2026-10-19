<?php

namespace App\Console\Commands\Pun;

use Symfony\Component\Console\Input\InputArgument;

class ConfigureChildCommand extends ConfigureBaseCommand
{
    protected $signature = 'pun:configure-child';

    protected function configure(): void
    {
        parent::configure();
        $this->getDefinition()->addArgument(new InputArgument('child-arg'));
        $this->{'setHidden'}(false);
    }

    public function handle(): void
    {
        $this->argument('base-arg');
        $this->argument('child-arg');
        $this->argument('nope');
    }
}
