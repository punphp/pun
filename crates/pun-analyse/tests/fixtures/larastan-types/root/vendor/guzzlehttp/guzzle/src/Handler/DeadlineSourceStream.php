<?php

declare(strict_types=1);

namespace GuzzleHttp\Handler;

use GuzzleHttp\NonSerializableTrait;
use GuzzleHttp\Psr7\Exception\TimeoutException;
use GuzzleHttp\Psr7\StreamDecoratorTrait;
use Psr\Http\Message\StreamInterface;

/**
 * Decorates the transport stream of a buffered response so every read
 * observes the wall-clock deadline of the "timeout" request option.
 *
 * The transport resource must be in non-blocking mode: a blocking read waits
 * for data up to the transport's own timeout, so a deadline check between reads
 * never runs while the server stalls. Reads return whatever bytes are available
 * and poll briefly for more, so the deadline is checked between packets rather
 * than between buffers.
 *
 * @internal
 */
final class DeadlineSourceStream implements StreamInterface
{
    use NonSerializableTrait;
    use StreamDecoratorTrait;

    private const POLL_INTERVAL = 0.05;

    private StreamInterface $stream;

    private float $deadline;

    private ?float $readTimeout;

    /**
     * @param StreamInterface $stream      Transport stream in non-blocking mode.
     * @param float           $deadline    Deadline based on Clock::now().
     * @param float|null      $readTimeout Optional idle timeout per read, in seconds.
     */
    public function __construct(StreamInterface $stream, float $deadline, ?float $readTimeout)
    {}

    public function read(int $length): string
    {}
}
