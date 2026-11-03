<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Mailer\Transport\Smtp\Stream;

use Symfony\Component\Mailer\Exception\TransportException;

/**
 * A stream supporting remote sockets and local processes.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 * @author Nicolas Grekas <p@tchwork.com>
 * @author Chris Corbyn
 *
 * @internal
 */
abstract class AbstractStream
{
    /** @var resource|null */
    protected $stream;
    /** @var resource|null */
    protected $in;
    /** @var resource|null */
    protected $out;
    protected $err;

    private string $debug = '';

    public function write(string $bytes, bool $debug = true): void
    {}

    /**
     * Flushes the contents of the stream (empty it) and set the internal pointer to the beginning.
     */
    public function flush(): void
    {}

    /**
     * Performs any initialization needed.
     */
    abstract public function initialize(): void;

    public function terminate(): void
    {}

    public function readLine(): string
    {}

    /**
     * Tells whether the server sent data that has not been read yet.
     */
    public function hasPendingData(): bool
    {}

    public function getDebug(): string
    {}

    public static function replace(string $from, string $to, iterable $chunks): \Generator
    { yield; }

    abstract protected function getReadConnectionDescription(): string;
}
