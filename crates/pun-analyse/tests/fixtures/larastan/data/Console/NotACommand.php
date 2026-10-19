<?php

namespace App\Console\Commands\Pun;

class NotACommand
{
    public function argument(string $name): void
    {
    }

    public function handle(ChildOneCommand $command): void
    {
        $this->argument('nope');
        $command->argument('nope');
    }
}
