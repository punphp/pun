<?php

namespace Illuminate\Foundation\Exceptions\Renderer\Mappers;

use Illuminate\Contracts\Foundation\Application;
use Illuminate\Contracts\View\Factory;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\View\Compilers\BladeCompiler;
use Illuminate\View\ViewException;
use ReflectionClass;
use ReflectionProperty;
use Symfony\Component\ErrorHandler\Exception\FlattenException;
use Throwable;

/*
 * This file contains parts of https://github.com/spatie/laravel-ignition.
 *
 * (c) Spatie <info@spatie.be>
 *
 * For the full copyright and license information, please review its LICENSE:
 *
 * The MIT License (MIT)
 *
 * Copyright (c) Spatie <info@spatie.be>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

class BladeMapper
{
    /**
     * The view factory instance.
     *
     * @var \Illuminate\Contracts\View\Factory
     */
    protected $factory;

    /**
     * The Blade compiler instance.
     *
     * @var \Illuminate\View\Compilers\BladeCompiler
     */
    protected $bladeCompiler;

    /**
     * Create a new Blade mapper instance.
     *
     * @param  \Illuminate\Contracts\View\Factory  $factory
     * @param  \Illuminate\View\Compilers\BladeCompiler  $bladeCompiler
     */
    public function __construct(Factory $factory, BladeCompiler $bladeCompiler)
    {}

    /**
     * Map cached view paths to their original paths.
     *
     * @param  \Symfony\Component\ErrorHandler\Exception\FlattenException  $exception
     * @return \Symfony\Component\ErrorHandler\Exception\FlattenException
     */
    public function map(FlattenException $exception)
    {}

    /**
     * Find the compiled view file for the given compiled path.
     *
     * @param  string  $compiledPath
     * @return string|null
     */
    public function findCompiledView(string $compiledPath)
    {}

    /**
     * Get the list of known paths from the compiler engine.
     *
     * @return array<string, string>
     */
    protected function getKnownPaths()
    {}

    /**
     * Filter out the view data that should not be shown in the exception report.
     *
     * @param  array<string, mixed>  $data
     * @return array<string, mixed>
     */
    protected function filterViewData(array $data)
    {}

    /**
     * Detect the line number in the original blade file.
     *
     * @param  string  $filename
     * @param  int  $compiledLineNumber
     * @return int
     */
    public function detectLineNumber(string $filename, int $compiledLineNumber)
    {}

    /**
     * Compile the source map for the given blade file.
     *
     * @param  string  $value
     * @return string
     */
    protected function compileSourcemap(string $value)
    {}

    /**
     * Add line numbers to echo statements.
     *
     * @param  string  $value
     * @return string
     */
    protected function addEchoLineNumbers(string $value)
    {}

    /**
     * Add line numbers to blade statements.
     *
     * @param  string  $value
     * @return string
     */
    protected function addStatementLineNumbers(string $value)
    {}

    /**
     * Add line numbers to blade components.
     *
     * @param  string  $value
     * @return string
     */
    protected function addBladeComponentLineNumbers(string $value)
    {}

    /**
     * Insert a line number at the given position.
     *
     * @param  int  $position
     * @param  string  $value
     * @return string
     */
    protected function insertLineNumberAtPosition(int $position, string $value)
    {}

    /**
     * Trim empty lines from the given value.
     *
     * @param  string  $value
     * @return string
     */
    protected function trimEmptyLines(string $value)
    {}

    /**
     * Find the closest line number mapping in the given source map.
     *
     * @param  string  $map
     * @param  int  $compiledLineNumber
     * @return int
     */
    protected function findClosestLineNumberMapping(string $map, int $compiledLineNumber)
    {}
}
