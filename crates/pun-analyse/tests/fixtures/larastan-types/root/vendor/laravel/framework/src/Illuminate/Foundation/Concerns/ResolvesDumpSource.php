<?php

namespace Illuminate\Foundation\Concerns;

use Illuminate\Support\Str;
use Throwable;

trait ResolvesDumpSource
{
    /**
     * All of the href formats for common editors.
     *
     * @var array<string, string>
     */
    protected $editorHrefs = [
        'antigravity' => 'antigravity://file/{file}:{line}',
        'atom' => 'atom://core/open/file?filename={file}&line={line}',
        'cursor' => 'cursor://file/{file}:{line}',
        'emacs' => 'emacs://open?url=file://{file}&line={line}',
        'fleet' => 'fleet://open?file={file}&line={line}',
        'idea' => 'idea://open?file={file}&line={line}',
        'kiro' => 'kiro://file/{file}:{line}',
        'macvim' => 'mvim://open/?url=file://{file}&line={line}',
        'neovim' => 'nvim://open?url=file://{file}&line={line}',
        'netbeans' => 'netbeans://open/?f={file}:{line}',
        'nova' => 'nova://core/open/file?filename={file}&line={line}',
        'phpstorm' => 'phpstorm://open?file={file}&line={line}',
        'sublime' => 'subl://open?url=file://{file}&line={line}',
        'textmate' => 'txmt://open?url=file://{file}&line={line}',
        'trae' => 'trae://file/{file}:{line}',
        'vscode' => 'vscode://file/{file}:{line}',
        'vscode-insiders' => 'vscode-insiders://file/{file}:{line}',
        'vscode-insiders-remote' => 'vscode-insiders://vscode-remote/{file}:{line}',
        'vscode-remote' => 'vscode://vscode-remote/{file}:{line}',
        'vscodium' => 'vscodium://file/{file}:{line}',
        'windsurf' => 'windsurf://file/{file}:{line}',
        'xdebug' => 'xdebug://{file}@{line}',
        'zed' => 'zed://file/{file}:{line}',
    ];

    /**
     * Files that require special trace handling and their levels.
     *
     * @var array<string, int>
     */
    protected static $adjustableTraces = [
        'symfony/var-dumper/Resources/functions/dump.php' => 1,
        'Illuminate/Collections/Traits/EnumeratesValues.php' => 4,
    ];

    /**
     * The source resolver.
     *
     * @var (callable(): (array{0: string, 1: string, 2: int|null}|null))|null|false
     */
    protected static $dumpSourceResolver;

    /**
     * Resolve the source of the dump call.
     *
     * @return array{0: string, 1: string, 2: int|null}|null
     */
    public function resolveDumpSource()
    {}

    /**
     * Determine if the given file is a view compiled.
     *
     * @param  string  $file
     * @return bool
     */
    protected function isCompiledViewFile($file)
    {}

    /**
     * Get the original view compiled file by the given compiled file.
     *
     * @param  string  $file
     * @return string
     */
    protected function getOriginalFileForCompiledView($file)
    {}

    /**
     * Resolve the source href, if possible.
     *
     * @param  string  $file
     * @param  int|null  $line
     * @return string|null
     */
    protected function resolveSourceHref($file, $line)
    {}

    /**
     * Set the resolver that resolves the source of the dump call.
     *
     * @param  (callable(): (array{0: string, 1: string, 2: int|null}|null))|null  $callable
     * @return void
     */
    public static function resolveDumpSourceUsing($callable)
    {}

    /**
     * Don't include the location / file of the dump in dumps.
     *
     * @return void
     */
    public static function dontIncludeSource()
    {}
}
