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
abstract class AbstractPart
{
    private Headers $headers;

    public function __construct()
    {}

    public function getHeaders(): Headers
    {}

    public function getPreparedHeaders(): Headers
    {}

    public function toString(): string
    {}

    public function toIterable(): iterable
    { yield; }

    public function asDebugString(): string
    {}

    abstract public function bodyToString(): string;

    abstract public function bodyToIterable(): iterable;

    abstract public function getMediaType(): string;

    abstract public function getMediaSubtype(): string;

    public function __serialize(): array
    {}

    public function __unserialize(array $data): void
    {}

    /**
     * Returns the properties declared by subclasses that are not part of Symfony, keyed like the native serialization does.
     */
    private function getSubclassState(): array
    {}

    private function setSubclassState(array $data): void
    {}

    private static function isCoreClass(string $class): bool
    {}

    private static function castsToString(\ReflectionProperty $property, object $value): bool
    {}
}
