<?php

/*
 * Copyright 2012 Johannes M. Schmitt <schmittjoh@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

namespace PhpOption;

use Traversable;

/**
 * @template-covariant T
 *
 * @extends Option<T>
 */
final class LazyOption extends Option
{
    /** @var callable(mixed...):(Option<T>) */
    private $callback;

    /** @var array<int, mixed> */
    private $arguments;

    /** @var Option<T>|null */
    private $option;

    /**
     * @template S
     * @param callable(mixed...):(Option<S>) $callback
     * @param array<int, mixed>              $arguments
     *
     * @return LazyOption<S>
     */
    public static function create($callback, array $arguments = []): self
    {}

    /**
     * @param callable(mixed...):(Option<T>) $callback
     * @param array<int, mixed>              $arguments
     *
     * @throws \InvalidArgumentException If the callback is not callable.
     */
    public function __construct($callback, array $arguments = [])
    {}

    public function isDefined(): bool
    {}

    public function isEmpty(): bool
    {}

    public function get()
    {}

    public function getOrElse($default)
    {}

    public function getOrCall($callable)
    {}

    public function getOrThrow(\Exception $ex)
    {}

    public function orElse(Option $else)
    {}

    public function ifDefined($callable)
    {}

    public function forAll($callable)
    {}

    public function map($callable)
    {}

    public function flatMap($callable)
    {}

    public function filter($callable)
    {}

    public function filterNot($callable)
    {}

    public function select($value)
    {}

    public function reject($value)
    {}

    /**
     * @return Traversable<T>
     */
    public function getIterator(): Traversable
    {}

    public function foldLeft($initialValue, $callable)
    {}

    public function foldRight($initialValue, $callable)
    {}

    /**
     * @return Option<T>
     */
    private function option(): Option
    {}
}
