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
 * @template-covariant E
 *
 * @extends \GrahamCampbell\ResultType\Result<never,E>
 */
final class Error extends Result
{
    /**
     * @var E
     */
    private $value;

    /**
     * Internal constructor for an error value.
     *
     * @param E $value
     *
     * @return void
     */
    private function __construct($value)
    {}

    /**
     * Create a new error value.
     *
     * @template F
     *
     * @param F $value
     *
     * @return \GrahamCampbell\ResultType\Error<F>
     */
    public static function create($value)
    {}

    /**
     * Get the success option value.
     *
     * @return \PhpOption\Option<never>
     */
    public function success()
    {}

    /**
     * Map over the success value.
     *
     * @template S
     *
     * @param callable(never):S $f
     *
     * @return \GrahamCampbell\ResultType\Error<E>
     */
    public function map(callable $f)
    {}

    /**
     * Flat map over the success value.
     *
     * @template S
     * @template F
     *
     * @param callable(never):\GrahamCampbell\ResultType\Result<S,F> $f
     *
     * @return \GrahamCampbell\ResultType\Error<E>
     */
    public function flatMap(callable $f)
    {}

    /**
     * Get the error option value.
     *
     * @return \PhpOption\Option<E>
     */
    public function error()
    {}

    /**
     * Map over the error value.
     *
     * @template F
     *
     * @param callable(E):F $f
     *
     * @return \GrahamCampbell\ResultType\Error<F>
     */
    public function mapError(callable $f)
    {}
}
