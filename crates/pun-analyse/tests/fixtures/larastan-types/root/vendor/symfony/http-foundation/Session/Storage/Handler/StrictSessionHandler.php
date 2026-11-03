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
 * Adds basic `SessionUpdateTimestampHandlerInterface` behaviors to another `SessionHandlerInterface`.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 */
class StrictSessionHandler extends AbstractSessionHandler
{
    private bool $doDestroy;

    public function __construct(
        private \SessionHandlerInterface $handler,
    ) {}

    /**
     * Returns true if this handler wraps an internal PHP session save handler using \SessionHandler.
     *
     * @internal
     */
    public function isWrapper(): bool
    {}

    public function open(string $savePath, string $sessionName): bool
    {}

    protected function doRead(#[\SensitiveParameter] string $sessionId): string|false
    {}

    public function updateTimestamp(#[\SensitiveParameter] string $sessionId, string $data): bool
    {}

    protected function doWrite(#[\SensitiveParameter] string $sessionId, string $data): bool
    {}

    public function destroy(#[\SensitiveParameter] string $sessionId): bool
    {}

    protected function doDestroy(#[\SensitiveParameter] string $sessionId): bool
    {}

    public function close(): bool
    {}

    public function gc(int $maxlifetime): int|false
    {}
}
