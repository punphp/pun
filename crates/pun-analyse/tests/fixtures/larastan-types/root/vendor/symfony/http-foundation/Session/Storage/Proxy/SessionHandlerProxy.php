<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\HttpFoundation\Session\Storage\Proxy;

use Symfony\Component\HttpFoundation\Session\Storage\Handler\StrictSessionHandler;

/**
 * @author Drak <drak@zikula.org>
 */
class SessionHandlerProxy extends AbstractProxy implements \SessionHandlerInterface, \SessionUpdateTimestampHandlerInterface
{
    public function __construct(
        protected \SessionHandlerInterface $handler,
    ) {}

    public function getHandler(): \SessionHandlerInterface
    {}

    // \SessionHandlerInterface

    public function open(string $savePath, string $sessionName): bool
    {}

    public function close(): bool
    {}

    public function read(#[\SensitiveParameter] string $sessionId): string|false
    {}

    public function write(#[\SensitiveParameter] string $sessionId, string $data): bool
    {}

    public function destroy(#[\SensitiveParameter] string $sessionId): bool
    {}

    public function gc(int $maxlifetime): int|false
    {}

    public function create_sid(): string
    {}

    public function validateId(#[\SensitiveParameter] string $sessionId): bool
    {}

    public function updateTimestamp(#[\SensitiveParameter] string $sessionId, string $data): bool
    {}
}
