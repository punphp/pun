<?php

namespace Illuminate\Foundation\Console;

use Illuminate\Foundation\Concerns\ResolvesDumpSource;
use Symfony\Component\Console\Output\ConsoleOutput;
use Symfony\Component\VarDumper\Caster\ReflectionCaster;
use Symfony\Component\VarDumper\Cloner\Data;
use Symfony\Component\VarDumper\Cloner\VarCloner;
use Symfony\Component\VarDumper\Dumper\CliDumper as BaseCliDumper;
use Symfony\Component\VarDumper\VarDumper;

class CliDumper extends BaseCliDumper
{
    use ResolvesDumpSource;

    /**
     * The base path of the application.
     *
     * @var string
     */
    protected $basePath;

    /**
     * The output instance.
     *
     * @var \Symfony\Component\Console\Output\OutputInterface
     */
    protected $output;

    /**
     * The compiled view path for the application.
     *
     * @var string
     */
    protected $compiledViewPath;

    /**
     * If the dumper is currently dumping.
     *
     * @var bool
     */
    protected $dumping = false;

    /**
     * Create a new CLI dumper instance.
     *
     * @param  \Symfony\Component\Console\Output\OutputInterface  $output
     * @param  string  $basePath
     * @param  string  $compiledViewPath
     */
    public function __construct($output, $basePath, $compiledViewPath)
    {}

    /**
     * Create a new CLI dumper instance and register it as the default dumper.
     *
     * @param  string  $basePath
     * @param  string  $compiledViewPath
     * @return void
     */
    public static function register($basePath, $compiledViewPath)
    {}

    /**
     * Dump a variable with its source file / line.
     *
     * @param  \Symfony\Component\VarDumper\Cloner\Data  $data
     * @return void
     */
    public function dumpWithSource(Data $data)
    {}

    /**
     * Get the dump's source console content.
     *
     * @return string
     */
    protected function getDumpSourceContent()
    {}

    /**
     * {@inheritDoc}
     */
    protected function supportsColors(): bool
    {}
}
