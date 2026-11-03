<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Mime\Part;

use Symfony\Component\Mime\Header\Headers;

/**
 * @author Fabien Potencier <fabien@symfony.com>
 */
abstract class AbstractMultipartPart extends AbstractPart
{
    private ?string $boundary = null;
    private array $parts = [];

    public function __construct(AbstractPart ...$parts)
    {}

    /**
     * @return AbstractPart[]
     */
    public function getParts(): array
    {}

    public function getMediaType(): string
    {}

    public function getPreparedHeaders(): Headers
    {}

    public function bodyToString(): string
    {}

    public function bodyToIterable(): iterable
    { yield; }

    public function asDebugString(): string
    {}

    public function __serialize(): array
    {}

    public function __unserialize(array $data): void
    {}

    private function getBoundary(): string
    {}
}
