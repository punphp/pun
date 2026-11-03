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

use ArrayIterator;

/**
 * @template-covariant T
 *
 * @extends Option<T>
 */
final class Some extends Option
{
    /** @var T */
    private $value;

    /**
     * @param T $value
     */
    public function __construct($value)
    {}

    /**
     * @template U
     *
     * @param U $value
     *
     * @return Some<U>
     */
    public static function create($value): self
    {}

    public function isDefined(): bool
    {}

    public function isEmpty(): bool
    {}

    public function get()
    {}

    /**
     * @param mixed $default
     *
     * @return T
     */
    public function getOrElse($default)
    {}

    /**
     * @param callable():mixed $callable
     *
     * @return T
     */
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
     * @return ArrayIterator<int, T>
     */
    public function getIterator(): ArrayIterator
    {}

    /**
     * @template S
     * @template R
     *
     * @param S                $initialValue
     * @param callable(S, T):R $callable
     *
     * @return R
     */
    public function foldLeft($initialValue, $callable)
    {}

    /**
     * @template S
     * @template R
     *
     * @param S                $initialValue
     * @param callable(T, S):R $callable
     *
     * @return R
     */
    public function foldRight($initialValue, $callable)
    {}
}
