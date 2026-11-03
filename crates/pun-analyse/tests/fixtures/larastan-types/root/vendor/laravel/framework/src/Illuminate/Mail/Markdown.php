<?php

namespace Illuminate\Mail;

use Illuminate\Contracts\View\Factory as ViewFactory;
use Illuminate\Support\EncodedHtmlString;
use Illuminate\Support\HtmlString;
use Illuminate\Support\Str;
use League\CommonMark\Environment\Environment;
use League\CommonMark\Extension\CommonMark\CommonMarkCoreExtension;
use League\CommonMark\Extension\Table\TableExtension;
use League\CommonMark\MarkdownConverter;
use TijsVerkoyen\CssToInlineStyles\CssToInlineStyles;

class Markdown
{
    /**
     * The view factory implementation.
     *
     * @var \Illuminate\Contracts\View\Factory
     */
    protected $view;

    /**
     * The current theme being used when generating emails.
     *
     * @var string
     */
    protected $theme = 'default';

    /**
     * The registered component paths.
     *
     * @var array
     */
    protected $componentPaths = [];

    /**
     * Indicates if secure encoding should be enabled.
     *
     * @var bool
     */
    protected static $withSecuredEncoding = false;

    /**
     * The registered CommonMark extensions.
     *
     * @var array<int, class-string<\League\CommonMark\Extension\ExtensionInterface>>
     */
    protected static $extensions = [];

    /**
     * Create a new Markdown renderer instance.
     *
     * @param  \Illuminate\Contracts\View\Factory  $view
     * @param  array  $options
     */
    public function __construct(ViewFactory $view, array $options = [])
    {}

    /**
     * Render the Markdown template into HTML.
     *
     * @param  string  $view
     * @param  array  $data
     * @param  \TijsVerkoyen\CssToInlineStyles\CssToInlineStyles|null  $inliner
     * @return \Illuminate\Support\HtmlString
     */
    public function render($view, array $data = [], $inliner = null)
    {}

    /**
     * Render the Markdown template into text.
     *
     * @param  string  $view
     * @param  array  $data
     * @return \Illuminate\Support\HtmlString
     */
    public function renderText($view, array $data = [])
    {}

    /**
     * Parse the given Markdown text into HTML.
     *
     * @param  string  $text
     * @param  bool  $encoded
     * @return \Illuminate\Support\HtmlString
     */
    public static function parse($text, bool $encoded = false)
    {}

    /**
     * Get a Markdown converter instance.
     *
     * @internal
     *
     * @param  array<string, mixed>  $config
     * @return \League\CommonMark\MarkdownConverter
     */
    public static function converter(array $config = [])
    {}

    /**
     * Get the HTML component paths.
     *
     * @return array
     */
    public function htmlComponentPaths()
    {}

    /**
     * Get the text component paths.
     *
     * @return array
     */
    public function textComponentPaths()
    {}

    /**
     * Get the component paths.
     *
     * @return array
     */
    protected function componentPaths()
    {}

    /**
     * Register new mail component paths.
     *
     * @param  array  $paths
     * @return void
     */
    public function loadComponentsFrom(array $paths = [])
    {}

    /**
     * Set the default theme to be used.
     *
     * @param  string  $theme
     * @return $this
     */
    public function theme($theme)
    {}

    /**
     * Get the theme currently being used by the renderer.
     *
     * @return string
     */
    public function getTheme()
    {}

    /**
     * Enable secured encoding when parsing Markdown.
     *
     * @return void
     */
    public static function withSecuredEncoding()
    {}

    /**
     * Disable secured encoding when parsing Markdown.
     *
     * @return void
     */
    public static function withoutSecuredEncoding()
    {}

    /**
     * Flush the class's global state.
     *
     * @return void
     */
    public static function flushState()
    {}
}
