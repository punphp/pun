<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\HttpFoundation;

/**
 * StreamedResponse represents a streamed HTTP response.
 *
 * A StreamedResponse uses a callback or an iterable of strings for its content.
 *
 * The callback should use the standard PHP functions like echo
 * to stream the response back to the client. The flush() function
 * can also be used if needed.
 *
 * @see flush()
 *
 * @author Fabien Potencier <fabien@symfony.com>
 */
class StreamedResponse extends Response
{
    protected ?\Closure $callback = null;
    protected bool $streamed = false;

    private bool $headersSent = false;

    /**
     * @param callable|iterable<string>|null $callbackOrChunks
     * @param int                            $status           The HTTP status code (200 "OK" by default)
     */
    public function __construct(callable|iterable|null $callbackOrChunks = null, int $status = 200, array $headers = [])
    {}

    /**
     * @param iterable<string> $chunks
     */
    public function setChunks(iterable $chunks): static
    {}

    /**
     * Sets the PHP callback associated with this Response.
     *
     * @return $this
     */
    public function setCallback(callable $callback): static
    {}

    public function getCallback(): ?\Closure
    {}

    /**
     * This method only sends the headers once.
     *
     * @param positive-int|null $statusCode The status code to use, override the statusCode property if set and not null
     *
     * @return $this
     */
    public function sendHeaders(?int $statusCode = null): static
    {}

    /**
     * This method only sends the content once.
     *
     * @return $this
     */
    public function sendContent(): static
    {}

    /**
     * @return $this
     *
     * @throws \LogicException when the content is not null
     */
    public function setContent(?string $content): static
    {}

    public function getContent(): string|false
    {}
}
