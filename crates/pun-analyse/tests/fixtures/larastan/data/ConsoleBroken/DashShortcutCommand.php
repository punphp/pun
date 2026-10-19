<?php

namespace App\Console\Commands\Pun;

use Illuminate\Console\Command;

class DashShortcutCommand extends Command
{
    protected $name = 'pun:dash-shortcut';

    protected function getOptions(): array
    {
        return [['dash', '-']];
    }

    public function handle(): void
    {
        $this->option('nope');
    }
}
