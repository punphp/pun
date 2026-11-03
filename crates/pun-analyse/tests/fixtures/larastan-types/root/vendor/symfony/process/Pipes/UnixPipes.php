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

use Symfony\Component\Process\Process;

/**
 * UnixPipes implementation uses unix pipes as handles.
 *
 * @author Romain Neutron <imprec@gmail.com>
 *
 * @internal
 */
class UnixPipes extends AbstractPipes
{
    public function __construct(
        private ?bool $ttyMode,
        private bool $ptyMode,
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
}
