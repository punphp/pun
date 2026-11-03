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

use Symfony\Component\Process\Exception\RuntimeException;
use Symfony\Component\Process\Process;

/**
 * WindowsPipes implementation uses temporary files as handles.
 *
 * @see https://bugs.php.net/51800
 * @see https://bugs.php.net/65650
 *
 * @author Romain Neutron <imprec@gmail.com>
 *
 * @internal
 */
class WindowsPipes extends AbstractPipes
{
    private array $files = [];
    private array $fileHandles = [];
    private array $lockHandles = [];
    private array $readBytes = [
        Process::STDOUT => 0,
        Process::STDERR => 0,
    ];

    public function __construct(
        mixed $input,
        private bool $haveReadSupport,
    ) {}

    public function __serialize(): array
    {}

    public function __unserialize(array $data): void
    {}

    public function __destruct()
    {}

    public function getDescriptors(): array
    {}

    public function getFiles(): array
    {}

    public function readAndWrite(bool $blocking, bool $close = false): array
    {}

    public function haveReadSupport(): bool
    {}

    public function areOpen(): bool
    {}

    public function close(): void
    {}
}
