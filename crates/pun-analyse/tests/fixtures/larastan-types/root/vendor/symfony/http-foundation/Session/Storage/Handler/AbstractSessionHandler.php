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

use Symfony\Component\HttpFoundation\Session\SessionUtils;

/**
 * This abstract session handler provides a generic implementation
 * of the PHP 7.0 SessionUpdateTimestampHandlerInterface,
 * enabling strict and lazy session handling.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 */
abstract class AbstractSessionHandler implements \SessionHandlerInterface, \SessionUpdateTimestampHandlerInterface
{
    private string $sessionName;
    private string $prefetchId;
    private string|false $prefetchData;
    private ?\Throwable $prefetchError = null;
    private ?string $newSessionId = null;
    private ?string $readSessionId = null;
    private string $igbinaryEmptyData;

    public function __serialize(): array
    {}

    public function __unserialize(array $data): void
    {}

    public function open(string $savePath, string $sessionName): bool
    {}

    public function create_sid(): string
    {}

    abstract protected function doRead(#[\SensitiveParameter] string $sessionId): string|false;

    abstract protected function doWrite(#[\SensitiveParameter] string $sessionId, string $data): bool;

    abstract protected function doDestroy(#[\SensitiveParameter] string $sessionId): bool;

    public function validateId(#[\SensitiveParameter] string $sessionId): bool
    {}

    public function read(#[\SensitiveParameter] string $sessionId): string|false
    {}

    public function updateTimestamp(#[\SensitiveParameter] string $sessionId, string $data): bool
    {}

    public function write(#[\SensitiveParameter] string $sessionId, string $data): bool
    {}

    public function destroy(#[\SensitiveParameter] string $sessionId): bool
    {}
}
