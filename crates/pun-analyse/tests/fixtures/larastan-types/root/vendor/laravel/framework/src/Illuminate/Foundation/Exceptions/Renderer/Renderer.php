<?php

namespace Illuminate\Foundation\Exceptions\Renderer;

use Illuminate\Contracts\View\Factory;
use Illuminate\Foundation\Exceptions\Renderer\Mappers\BladeMapper;
use Illuminate\Http\Request;
use Symfony\Component\ErrorHandler\ErrorRenderer\HtmlErrorRenderer;
use Throwable;

class Renderer
{
    /**
     * The path to the renderer's distribution files.
     *
     * @var string
     */
    protected const DIST = __DIR__.'/../../resources/exceptions/renderer/dist/';

    /**
     * The view factory instance.
     *
     * @var \Illuminate\Contracts\View\Factory
     */
    protected $viewFactory;

    /**
     * The exception listener instance.
     *
     * @var \Illuminate\Foundation\Exceptions\Renderer\Listener
     */
    protected $listener;

    /**
     * The HTML error renderer instance.
     *
     * @var \Symfony\Component\ErrorHandler\ErrorRenderer\HtmlErrorRenderer
     */
    protected $htmlErrorRenderer;

    /**
     * The Blade mapper instance.
     *
     * @var \Illuminate\Foundation\Exceptions\Renderer\Mappers\BladeMapper
     */
    protected $bladeMapper;

    /**
     * The application's base path.
     *
     * @var string
     */
    protected $basePath;

    /**
     * Creates a new exception renderer instance.
     *
     * @param  \Illuminate\Contracts\View\Factory  $viewFactory
     * @param  \Illuminate\Foundation\Exceptions\Renderer\Listener  $listener
     * @param  \Symfony\Component\ErrorHandler\ErrorRenderer\HtmlErrorRenderer  $htmlErrorRenderer
     * @param  \Illuminate\Foundation\Exceptions\Renderer\Mappers\BladeMapper  $bladeMapper
     * @param  string  $basePath
     */
    public function __construct(
        Factory $viewFactory,
        Listener $listener,
        HtmlErrorRenderer $htmlErrorRenderer,
        BladeMapper $bladeMapper,
        string $basePath,
    ) {}

    /**
     * Render the given exception as an HTML string.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Throwable  $throwable
     * @return string
     */
    public function render(Request $request, Throwable $throwable)
    {}

    /**
     * Get the renderer's CSS content.
     *
     * @return string
     */
    public static function css()
    {}

    /**
     * Get the renderer's JavaScript content.
     *
     * @return string
     */
    public static function js()
    {}
}
