<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Helper;

use Symfony\Component\Console\Exception\InvalidArgumentException;

/**
 * HelperSet represents a set of helpers to be used with a command.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 *
 * @implements \IteratorAggregate<string, HelperInterface>
 */
class HelperSet implements \IteratorAggregate
{
    /** @var array<string, HelperInterface> */
    private array $helpers = [];

    /**
     * @param HelperInterface[] $helpers
     */
    public function __construct(array $helpers = [])
    {}

    public function set(HelperInterface $helper, ?string $alias = null): void
    {}

    /**
     * Returns true if the helper if defined.
     */
    public function has(string $name): bool
    {}

    /**
     * Gets a helper value.
     *
     * @throws InvalidArgumentException if the helper is not defined
     */
    public function get(string $name): HelperInterface
    {}

    public function getIterator(): \Traversable
    {}
}
