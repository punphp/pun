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

use EmptyIterator;

/**
 * @extends Option<never>
 */
final class None extends Option
{
    /** @var None|null */
    private static $instance;

    /**
     * @return None
     */
    public static function create(): self
    {}

    public function get()
    {}

    /**
     * @template S
     *
     * @param callable():S $callable
     *
     * @return S
     */
    public function getOrCall($callable)
    {}

    /**
     * @template S
     *
     * @param S $default
     *
     * @return S
     */
    public function getOrElse($default)
    {}

    public function getOrThrow(\Exception $ex)
    {}

    public function isEmpty(): bool
    {}

    public function isDefined(): bool
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

    public function getIterator(): EmptyIterator
    {}

    /**
     * @template S
     * @template R
     *
     * @param S                    $initialValue
     * @param callable(S, never):R $callable
     *
     * @return S
     */
    public function foldLeft($initialValue, $callable)
    {}

    /**
     * @template S
     * @template R
     *
     * @param S                    $initialValue
     * @param callable(never, S):R $callable
     *
     * @return S
     */
    public function foldRight($initialValue, $callable)
    {}

    private function __construct()
    {}
}
