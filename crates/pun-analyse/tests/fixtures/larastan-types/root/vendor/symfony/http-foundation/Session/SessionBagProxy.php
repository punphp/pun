<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\HttpFoundation\Session;

/**
 * @author Nicolas Grekas <p@tchwork.com>
 *
 * @internal
 */
final class SessionBagProxy implements SessionBagInterface
{
    private array $data;
    private ?int $usageIndex;
    private ?\Closure $usageReporter;

    public function __construct(
        private SessionBagInterface $bag,
        array &$data,
        ?int &$usageIndex,
        ?callable $usageReporter,
    ) {}

    public function getBag(): SessionBagInterface
    {}

    public function isEmpty(): bool
    {}

    public function getName(): string
    {}

    public function initialize(array &$array): void
    {}

    public function getStorageKey(): string
    {}

    public function clear(): mixed
    {}
}
