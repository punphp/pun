<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Process\Pipes;

use Symfony\Component\Process\Exception\InvalidArgumentException;

/**
 * @author Romain Neutron <imprec@gmail.com>
 *
 * @internal
 */
abstract class AbstractPipes implements PipesInterface
{
    public array $pipes = [];

    private string $inputBuffer = '';
    /** @var resource|string|\Iterator */
    private $input;
    private bool $blocked = true;
    private ?string $lastError = null;

    /**
     * @param resource|string|\Iterator $input
     */
    public function __construct($input)
    {}

    public function close(): void
    {}

    /**
     * Returns true if a system call has been interrupted.
     *
     * stream_select() returns false when the `select` system call is interrupted by an incoming signal.
     */
    protected function hasSystemCallBeenInterrupted(): bool
    {}

    /**
     * Unblocks streams.
     */
    protected function unblock(): void
    {}

    /**
     * Writes input to stdin.
     *
     * @throws InvalidArgumentException When an input iterator yields a non supported value
     */
    protected function write(): ?array
    {}

    /**
     * Tells whether the next chunk of input can be written without waiting.
     */
    private function hasReadyInput(): bool
    {}

    private function closeBrokenInputPipe(): void
    {}

    /**
     * @internal
     */
    public function handleError(int $type, string $msg): void
    {}
}
