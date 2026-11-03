<?php

declare(strict_types=1);

/**
 * This file is part of the Carbon package.
 *
 * (c) Brian Nesbitt <brian@nesbot.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Carbon\Exceptions;

use InvalidArgumentException as BaseInvalidArgumentException;
use Throwable;

class ParseErrorException extends BaseInvalidArgumentException implements InvalidArgumentException
{
    /**
     * The expected.
     *
     * @var string
     */
    protected $expected;

    /**
     * The actual.
     *
     * @var string
     */
    protected $actual;

    /**
     * The help message.
     *
     * @var string
     */
    protected $help;

    /**
     * Constructor.
     *
     * @param string         $expected
     * @param string         $actual
     * @param int            $code
     * @param Throwable|null $previous
     */
    public function __construct($expected, $actual, $help = '', $code = 0, ?Throwable $previous = null)
    {}

    /**
     * Get the expected.
     *
     * @return string
     */
    public function getExpected(): string
    {}

    /**
     * Get the actual.
     *
     * @return string
     */
    public function getActual(): string
    {}

    /**
     * Get the help message.
     *
     * @return string
     */
    public function getHelp(): string
    {}
}
