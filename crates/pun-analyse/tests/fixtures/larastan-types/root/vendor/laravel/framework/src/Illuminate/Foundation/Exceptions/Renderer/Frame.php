<?php

namespace Illuminate\Foundation\Exceptions\Renderer;

use Illuminate\Foundation\Concerns\ResolvesDumpSource;
use Symfony\Component\ErrorHandler\Exception\FlattenException;

use function Illuminate\Filesystem\join_paths;

class Frame
{
    use ResolvesDumpSource;

    /**
     * The "flattened" exception instance.
     *
     * @var \Symfony\Component\ErrorHandler\Exception\FlattenException
     */
    protected $exception;

    /**
     * The application's class map.
     *
     * @var array<string, string>
     */
    protected $classMap;

    /**
     * The frame's raw data from the "flattened" exception.
     *
     * @var array{file: string, line: int, class?: string, type?: string, function?: string}
     */
    protected $frame;

    /**
     * The application's base path.
     *
     * @var string
     */
    protected $basePath;

    /**
     * The previous frame.
     *
     * @var \Illuminate\Foundation\Exceptions\Renderer\Frame|null
     */
    protected $previous;

    /**
     * Whether this frame is the main (first non-vendor) frame.
     *
     * @var bool
     */
    protected $isMain = false;

    /**
     * Create a new frame instance.
     *
     * @param  \Symfony\Component\ErrorHandler\Exception\FlattenException  $exception
     * @param  array<string, string>  $classMap
     * @param  array{file: string, line: int, class?: string, type?: string, function?: string, args?: array}  $frame
     * @param  string  $basePath
     * @param  \Illuminate\Foundation\Exceptions\Renderer\Frame|null  $previous
     */
    public function __construct(FlattenException $exception, array $classMap, array $frame, string $basePath, ?Frame $previous = null)
    {}

    /**
     * Get the frame's source / origin.
     *
     * @return string
     */
    public function source()
    {}

    /**
     * Get the frame's editor link.
     *
     * @return string
     */
    public function editorHref()
    {}

    /**
     * Get the frame's class, if any.
     *
     * @return string|null
     */
    public function class()
    {}

    /**
     * Get the frame's file.
     *
     * @return string
     */
    public function file()
    {}

    /**
     * Get the frame's line number.
     *
     * @return int
     */
    public function line()
    {}

    /**
     * Get the frame's function operator.
     *
     * @return '::'|'->'|''
     */
    public function operator()
    {}

    /**
     * Get the frame's function or method.
     *
     * @return string
     */
    public function callable()
    {}

    /**
     * Get the frame's arguments.
     *
     * @return array
     */
    public function args()
    {}

    /**
     * Get the frame's code snippet.
     *
     * @return string
     */
    public function snippet()
    {}

    /**
     * Determine if the frame is from the vendor directory.
     *
     * @return bool
     */
    public function isFromVendor()
    {}

    /**
     * Get the previous frame.
     *
     * @return \Illuminate\Foundation\Exceptions\Renderer\Frame|null
     */
    public function previous()
    {}

    /**
     * Mark this frame as the main frame.
     *
     * @return void
     */
    public function markAsMain()
    {}

    /**
     * Determine if this is the main frame.
     *
     * @return bool
     */
    public function isMain()
    {}
}
