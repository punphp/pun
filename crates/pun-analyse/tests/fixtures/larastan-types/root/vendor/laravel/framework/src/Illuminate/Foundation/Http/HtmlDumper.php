<?php

namespace Illuminate\Foundation\Http;

use Illuminate\Foundation\Concerns\ResolvesDumpSource;
use Symfony\Component\VarDumper\Caster\ReflectionCaster;
use Symfony\Component\VarDumper\Cloner\Data;
use Symfony\Component\VarDumper\Cloner\VarCloner;
use Symfony\Component\VarDumper\Dumper\HtmlDumper as BaseHtmlDumper;
use Symfony\Component\VarDumper\VarDumper;

class HtmlDumper extends BaseHtmlDumper
{
    use ResolvesDumpSource;

    /**
     * Where the source should be placed on "expanded" kind of dumps.
     *
     * @var string
     */
    const EXPANDED_SEPARATOR = 'class=sf-dump-expanded>';

    /**
     * Where the source should be placed on "non expanded" kind of dumps.
     *
     * @var string
     */
    const NON_EXPANDED_SEPARATOR = "\n</pre><script>";

    /**
     * The base path of the application.
     *
     * @var string
     */
    protected $basePath;

    /**
     * The compiled view path of the application.
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
     * Create a new HTML dumper instance.
     *
     * @param  string  $basePath
     * @param  string  $compiledViewPath
     */
    public function __construct($basePath, $compiledViewPath)
    {}

    /**
     * Create a new HTML dumper instance and register it as the default dumper.
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
     * Get the dump's source HTML content.
     *
     * @return string
     */
    protected function getDumpSourceContent()
    {}
}
