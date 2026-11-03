<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

use Psr\Http\Message\StreamInterface;

/**
 * Converts Guzzle streams into PHP stream resources.
 *
 * @see https://www.php.net/streamwrapper
 */
final class StreamWrapper
{
    /** @var resource */
    public $context;

    private StreamInterface $stream;

    /** @var string r, r+, or w */
    private string $mode;

    /**
     * Returns a resource representing the stream.
     *
     * @param StreamInterface $stream The stream to get a resource for
     *
     * @return resource
     *
     * @throws \InvalidArgumentException if stream is not readable or writable
     */
    public static function getResource(StreamInterface $stream)
    {}

    /**
     * Creates a stream context that can be used to open a stream as a php stream resource.
     *
     * @return resource
     */
    public static function createStreamContext(StreamInterface $stream)
    {}

    /**
     * Registers the stream wrapper if needed
     */
    public static function register(): void
    {}

    public function stream_open(string $path, string $mode, int $options, ?string &$opened_path = null): bool
    {}

    /**
     * @return string|false
     */
    public function stream_read(int $count)
    {}

    public function stream_write(string $data): int
    {}

    public function stream_flush(): bool
    {}

    /**
     * @return int|false
     */
    public function stream_tell()
    {}

    public function stream_eof(): bool
    {}

    public function stream_seek(int $offset, int $whence): bool
    {}

    /**
     * @return resource|false
     */
    public function stream_cast(int $cast_as)
    {}

    /**
     * @return array{
     *   dev: int,
     *   ino: int,
     *   mode: int,
     *   nlink: int,
     *   uid: int,
     *   gid: int,
     *   rdev: int,
     *   size: int,
     *   atime: int,
     *   mtime: int,
     *   ctime: int,
     *   blksize: int,
     *   blocks: int
     * }|false
     */
    public function stream_stat()
    {}

    /**
     * @return array{
     *   dev: int,
     *   ino: int,
     *   mode: int,
     *   nlink: int,
     *   uid: int,
     *   gid: int,
     *   rdev: int,
     *   size: int,
     *   atime: int,
     *   mtime: int,
     *   ctime: int,
     *   blksize: int,
     *   blocks: int
     * }
     */
    public function url_stat(string $path, int $flags): array
    {}
}
