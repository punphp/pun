<?php

namespace Illuminate\Foundation\Events;

use Illuminate\Contracts\Events\ShouldBeDiscovered;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Reflector;
use Illuminate\Support\Str;
use ReflectionClass;
use ReflectionException;
use ReflectionMethod;
use SplFileInfo;
use Symfony\Component\Finder\Finder;

class DiscoverEvents
{
    /**
     * The callback to be used to guess class names.
     *
     * @var (callable(SplFileInfo, string): class-string)|null
     */
    public static $guessClassNamesUsingCallback;

    /**
     * Get all of the events and listeners by searching the given listener directory.
     *
     * @param  array<int, string>|string  $listenerPath
     * @param  string  $basePath
     * @return array
     */
    public static function within($listenerPath, $basePath)
    {}

    /**
     * Get all of the listeners and their corresponding events.
     *
     * @param  iterable<string, SplFileInfo>  $listeners
     * @param  string  $basePath
     * @return array
     */
    protected static function getListenerEvents($listeners, $basePath)
    {}

    /**
     * Extract the class name from the given file path.
     *
     * @param  \SplFileInfo  $file
     * @param  string  $basePath
     * @return class-string
     */
    protected static function classFromFile(SplFileInfo $file, $basePath)
    {}

    /**
     * Specify a callback to be used to guess class names.
     *
     * @param  callable(SplFileInfo, string): class-string  $callback
     * @return void
     */
    public static function guessClassNamesUsing(callable $callback)
    {}
}
