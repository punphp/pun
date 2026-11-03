<?php

declare(strict_types=1);

/*
 * This file is part of Result Type.
 *
 * (c) Graham Campbell <hello@gjcampbell.co.uk>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace GrahamCampbell\ResultType;

use PhpOption\None;
use PhpOption\Some;

/**
 * @template-covariant T
 *
 * @extends \GrahamCampbell\ResultType\Result<T,never>
 */
final class Success extends Result
{
    /**
     * @var T
     */
    private $value;

    /**
     * Internal constructor for a success value.
     *
     * @param T $value
     *
     * @return void
     */
    private function __construct($value)
    {}

    /**
     * Create a new success value.
     *
     * @template S
     *
     * @param S $value
     *
     * @return \GrahamCampbell\ResultType\Success<S>
     */
    public static function create($value)
    {}

    /**
     * Get the success option value.
     *
     * @return \PhpOption\Option<T>
     */
    public function success()
    {}

    /**
     * Map over the success value.
     *
     * @template S
     *
     * @param callable(T):S $f
     *
     * @return \GrahamCampbell\ResultType\Success<S>
     */
    public function map(callable $f)
    {}

    /**
     * Flat map over the success value.
     *
     * @template S
     * @template F
     *
     * @param callable(T):\GrahamCampbell\ResultType\Result<S,F> $f
     *
     * @return \GrahamCampbell\ResultType\Result<S,F>
     */
    public function flatMap(callable $f)
    {}

    /**
     * Get the error option value.
     *
     * @return \PhpOption\Option<never>
     */
    public function error()
    {}

    /**
     * Map over the error value.
     *
     * @template F
     *
     * @param callable(never):F $f
     *
     * @return \GrahamCampbell\ResultType\Success<T>
     */
    public function mapError(callable $f)
    {}
}
