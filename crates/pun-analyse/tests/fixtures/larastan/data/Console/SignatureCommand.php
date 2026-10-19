<?php

namespace App\Console\Commands\Pun;

use Illuminate\Console\Command;

class SignatureCommand extends Command
{
    protected $signature = 'pun:signature {user} {other?} {--Q|queue} {--D|delay=} {--A|all|every}
        {ids?*} {--tag=* : The tags}';

    public function handle(): void
    {
        $this->argument('user');
        $this->argument('other');
        $this->argument('ids');
        $this->argument('missing');
        $this->argument('command');
        $this->option('queue');
        $this->option('Q');
        $this->option('delay');
        $this->option('D');
        $this->option('A');
        $this->option('every');
        $this->option('all');
        $this->option('tag');
        $this->option('missing');
        $this->option('help');
        $this->option('h');
        $this->option('v');
        $this->option('vv');
        $this->option('vvv');
        $this->option('V');
        $this->option('q');
        $this->option('n');
        $this->option('silent');
        $this->option('ansi');
        $this->option('no-ansi');
        $this->option('env');
        $this->option('isolated');
        $this->option('--queue');
        $this->argument('user', 'x');
        $this->argument();
        $name = 'user';
        $this->argument($name);
        $this->argument(rand(0, 1) ? 'user' : 'nope');
        $other = new self();
        $other->option('nope');
        $this->Argument('nope');
    }
}
