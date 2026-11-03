<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Routing;

use Symfony\Component\Routing\Exception\InvalidArgumentException;

class Alias
{
    private array $deprecation = [];

    public function __construct(
        private string $id,
    ) {}

    public function withId(string $id): static
    {}

    /**
     * Returns the target name of this alias.
     *
     * @return string The target name
     */
    public function getId(): string
    {}

    /**
     * Whether this alias is deprecated, that means it should not be referenced anymore.
     *
     * @param string $package The name of the composer package that is triggering the deprecation
     * @param string $version The version of the package that introduced the deprecation
     * @param string $message The deprecation message to use
     *
     * @return $this
     *
     * @throws InvalidArgumentException when the message template is invalid
     */
    public function setDeprecated(string $package, string $version, string $message): static
    {}

    public function isDeprecated(): bool
    {}

    /**
     * @param string $name Route name relying on this alias
     */
    public function getDeprecation(string $name): array
    {}
}
