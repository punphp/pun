<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\ErrorHandler\Exception;

/**
 * Data Object that represents a Silenced Error.
 *
 * @author Grégoire Pineau <lyrixx@lyrixx.info>
 */
class SilencedErrorContext implements \JsonSerializable
{
    public int $count = 1;

    public function __construct(
        private int $severity,
        private string $file,
        private int $line,
        private array $trace = [],
        int $count = 1,
    ) {}

    public function getSeverity(): int
    {}

    public function getFile(): string
    {}

    public function getLine(): int
    {}

    public function getTrace(): array
    {}

    public function jsonSerialize(): array
    {}
}
