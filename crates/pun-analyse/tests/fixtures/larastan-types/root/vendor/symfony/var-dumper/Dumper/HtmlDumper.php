<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\VarDumper\Dumper;

use Symfony\Component\VarDumper\Cloner\Cursor;
use Symfony\Component\VarDumper\Cloner\Data;

/**
 * HtmlDumper dumps variables as HTML.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 */
class HtmlDumper extends CliDumper
{
    /** @var callable|resource|string|null */
    public static $defaultOutput = 'php://output';

    protected static $themes = [
        'dark' => [
            'default' => 'background-color:#18171B; color:#FF8400; line-height:1.2em; font:12px Menlo, Monaco, Consolas, monospace; word-wrap: break-word; white-space: pre-wrap; position:relative; z-index:99999; word-break: break-all',
            'num' => 'font-weight:bold; color:#1299DA',
            'const' => 'font-weight:bold',
            'virtual' => 'font-style:italic',
            'str' => 'font-weight:bold; color:#56DB3A',
            'note' => 'color:#1299DA',
            'ref' => 'color:#A0A0A0',
            'public' => 'color:#FFFFFF',
            'protected' => 'color:#FFFFFF',
            'private' => 'color:#FFFFFF',
            'meta' => 'color:#B729D9',
            'key' => 'color:#56DB3A',
            'index' => 'color:#1299DA',
            'ellipsis' => 'color:#FF8400',
            'ns' => 'user-select:none;',
        ],
        'light' => [
            'default' => 'background:none; color:#CC7832; line-height:1.2em; font:12px Menlo, Monaco, Consolas, monospace; word-wrap: break-word; white-space: pre-wrap; position:relative; z-index:99999; word-break: break-all',
            'num' => 'font-weight:bold; color:#1299DA',
            'const' => 'font-weight:bold',
            'virtual' => 'font-style:italic',
            'str' => 'font-weight:bold; color:#629755;',
            'note' => 'color:#6897BB',
            'ref' => 'color:#6E6E6E',
            'public' => 'color:#262626',
            'protected' => 'color:#262626',
            'private' => 'color:#262626',
            'meta' => 'color:#B729D9',
            'key' => 'color:#789339',
            'index' => 'color:#1299DA',
            'ellipsis' => 'color:#CC7832',
            'ns' => 'user-select:none;',
        ],
    ];

    protected ?string $dumpHeader = null;
    protected string $dumpPrefix = '<pre class=sf-dump id=%s data-indent-pad="%s">';
    protected string $dumpSuffix = '</pre><script>Sfdump(%s)</script>';
    protected string $dumpId;
    protected bool $colors = true;
    protected $headerIsDumped = false;
    protected int $lastDepth = -1;

    private array $displayOptions = [
        'maxDepth' => 1,
        'maxStringLength' => 160,
        'fileLinkFormat' => null,
    ];
    private array $extraDisplayOptions = [];
    private string|\Closure|null $nonce = null;
    private string|\Closure|null $styleNonce = null;

    public function __construct($output = null, ?string $charset = null, int $flags = 0)
    {}

    public function setStyles(array $styles): void
    {}

    public function setTheme(string $themeName): void
    {}

    /**
     * Configures display options.
     *
     * @param array $displayOptions A map of display options to customize the behavior
     */
    public function setDisplayOptions(array $displayOptions): void
    {}

    /**
     * Sets an HTML header that will be dumped once in the output stream.
     */
    public function setDumpHeader(?string $header): void
    {}

    /**
     * Sets an HTML prefix and suffix that will encapse every single dump.
     */
    public function setDumpBoundaries(string $prefix, string $suffix): void
    {}

    /**
     * Sets nonces to be added to <script> and <style> tags for CSP compliance.
     *
     * Pass a string for a static nonce, or a {@see \Closure} returning a
     * string (or null) to resolve the nonce lazily — useful for per-request
     * nonces in long-running processes (the closure is invoked on every
     * dump, so it can return a fresh value each time).
     *
     * If $styleNonce is omitted, $nonce is reused for both <script> and
     * <style> tags; pass a distinct value when your CSP uses different
     * nonces for the script-src and style-src directives.
     */
    public function setNonce(string|\Closure|null $nonce, string|\Closure|null $styleNonce = null): void
    {}

    private function applyNonce(string $html): string
    {}

    private function resolveNonce(string|\Closure|null $nonce): ?string
    {}

    public function dump(Data $data, $output = null, array $extraDisplayOptions = []): ?string
    {}

    /**
     * Dumps the HTML header.
     */
    protected function getDumpHeader(): string
    {}

    public function dumpString(Cursor $cursor, string $str, bool $bin, int $cut): void
    {}

    public function enterHash(Cursor $cursor, int $type, string|int|null $class, bool $hasChild): void
    {}

    public function leaveHash(Cursor $cursor, int $type, string|int|null $class, bool $hasChild, int $cut): void
    {}

    protected function style(string $style, string $value, array $attr = []): string
    {}

    protected function dumpLine(int $depth, bool $endOfValue = false): void
    {}

    private function getSourceLink(string $file, int $line): string|false
    {}
}

function esc(string $str): string
{}
