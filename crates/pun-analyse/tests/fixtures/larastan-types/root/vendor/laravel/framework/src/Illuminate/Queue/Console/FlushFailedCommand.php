<?php

namespace Illuminate\Queue\Console;

use Illuminate\Console\Command;
use Illuminate\Console\Prohibitable;
use Symfony\Component\Console\Attribute\AsCommand;

#[AsCommand(name: 'queue:flush')]
class FlushFailedCommand extends Command
{
    use Prohibitable;

    /**
     * The console command name.
     *
     * @var string
     */
    protected $signature = 'queue:flush
                            {--hours= : The number of hours to retain failed job data}
                            {--queue= : Flush all of the failed jobs for the specified queue}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Flush the failed queue jobs';

    /**
     * Execute the console command.
     *
     * @return void
     */
    public function handle()
    {}
}
