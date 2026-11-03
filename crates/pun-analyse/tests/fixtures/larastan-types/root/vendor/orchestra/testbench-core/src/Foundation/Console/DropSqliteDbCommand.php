<?php

namespace Orchestra\Testbench\Foundation\Console;

use Illuminate\Console\Command;
use Illuminate\Filesystem\Filesystem;
use Illuminate\Support\Str;
use Symfony\Component\Console\Attribute\AsCommand;

use function Orchestra\Sidekick\Filesystem\join_paths;

#[AsCommand(name: 'package:drop-sqlite-db', description: 'Drop sqlite database file')]
class DropSqliteDbCommand extends Command
{
    /**
     * The name and signature of the console command.
     *
     * @var string
     */
    protected $signature = 'package:drop-sqlite-db
                                {--database=database.sqlite : Set the database name}
                                {--all : Delete all SQLite databases}
                                {--pretend : Outputs the operations but will not execute anything}';

    /**
     * Execute the console command.
     *
     * @param  \Illuminate\Filesystem\Filesystem  $filesystem
     * @return int
     */
    public function handle(Filesystem $filesystem)
    {}

    /**
     * Resolve the database name.
     *
     * @return string
     */
    protected function databaseName(): string
    {}
}
