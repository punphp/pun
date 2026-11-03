<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\HttpFoundation\Session\Storage\Handler;

/**
 * Migrating session handler for migrating from one handler to another. It reads
 * from the current handler and writes both the current and new ones.
 *
 * It ignores errors from the new handler.
 *
 * @author Ross Motley <ross.motley@amara.com>
 * @author Oliver Radwell <oliver.radwell@amara.com>
 */
class MigratingSessionHandler implements \SessionHandlerInterface, \SessionUpdateTimestampHandlerInterface
{
    private \SessionHandlerInterface&\SessionUpdateTimestampHandlerInterface $currentHandler;
    private \SessionHandlerInterface&\SessionUpdateTimestampHandlerInterface $writeOnlyHandler;

    public function __construct(\SessionHandlerInterface $currentHandler, \SessionHandlerInterface $writeOnlyHandler)
    {}

    public function __serialize(): array
    {}

    public function __unserialize(array $data): void
    {}

    public function create_sid(): string
    {}

    public function close(): bool
    {}

    public function destroy(#[\SensitiveParameter] string $sessionId): bool
    {}

    public function gc(int $maxlifetime): int|false
    {}

    public function open(string $savePath, string $sessionName): bool
    {}

    public function read(#[\SensitiveParameter] string $sessionId): string|false
    {}

    public function write(#[\SensitiveParameter] string $sessionId, string $sessionData): bool
    {}

    public function validateId(#[\SensitiveParameter] string $sessionId): bool
    {}

    public function updateTimestamp(#[\SensitiveParameter] string $sessionId, string $sessionData): bool
    {}
}
