<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\HttpFoundation\Session\Flash;

/**
 * FlashBag flash message container.
 *
 * @author Drak <drak@zikula.org>
 */
class FlashBag implements FlashBagInterface
{
    private string $name = 'flashes';
    private array $flashes = [];

    /**
     * @param string $storageKey The key used to store flashes in the session
     */
    public function __construct(
        private string $storageKey = '_symfony_flashes',
    ) {}

    public function getName(): string
    {}

    public function setName(string $name): void
    {}

    public function initialize(array &$flashes): void
    {}

    public function add(string $type, mixed $message): void
    {}

    public function peek(string $type, array $default = []): array
    {}

    public function peekAll(): array
    {}

    public function get(string $type, array $default = []): array
    {}

    public function all(): array
    {}

    public function set(string $type, string|array $messages): void
    {}

    public function setAll(array $messages): void
    {}

    public function has(string $type): bool
    {}

    public function keys(): array
    {}

    public function getStorageKey(): string
    {}

    public function clear(): mixed
    {}
}
