<?php

namespace Illuminate\Redis\Connections;

use Redis;
use RuntimeException;
use UnexpectedValueException;

trait PacksPhpRedisValues
{
    /**
     * Indicates if Redis supports packing.
     *
     * @var bool|null
     */
    protected $supportsPacking;

    /**
     * Indicates if Redis supports LZF compression.
     *
     * @var bool|null
     */
    protected $supportsLzf;

    /**
     * Indicates if Redis supports Zstd compression.
     *
     * @var bool|null
     */
    protected $supportsZstd;

    /**
     * Prepares the given values to be used with the `eval` command, including serialization and compression.
     *
     * @param  array<int|string,string>  $values
     * @return array<int|string,string>
     *
     * @throws \RuntimeException
     * @throws \UnexpectedValueException
     */
    public function pack(array $values): array
    {}

    /**
     * Execute the given callback without serialization or compression when applicable.
     *
     * @template TReturn
     *
     * @param  (callable(): TReturn)  $callback
     * @return TReturn
     */
    public function withoutSerializationOrCompression(callable $callback)
    {}

    /**
     * Determine if serialization is enabled.
     *
     * @return bool
     */
    public function serialized(): bool
    {}

    /**
     * Determine if compression is enabled.
     *
     * @return bool
     */
    public function compressed(): bool
    {}

    /**
     * Determine if LZF compression is enabled.
     *
     * @return bool
     */
    public function lzfCompressed(): bool
    {}

    /**
     * Determine if ZSTD compression is enabled.
     *
     * @return bool
     */
    public function zstdCompressed(): bool
    {}

    /**
     * Determine if LZ4 compression is enabled.
     *
     * @return bool
     */
    public function lz4Compressed(): bool
    {}

    /**
     * Determine if the current PhpRedis extension version supports packing.
     *
     * @return bool
     */
    protected function supportsPacking(): bool
    {}

    /**
     * Determine if the current PhpRedis extension version supports LZF compression.
     *
     * @return bool
     */
    protected function supportsLzf(): bool
    {}

    /**
     * Determine if the current PhpRedis extension version supports Zstd compression.
     *
     * @return bool
     */
    protected function supportsZstd(): bool
    {}

    /**
     * Determine if the PhpRedis extension version is at least the given version.
     *
     * @param  string  $version
     * @return bool
     */
    protected function phpRedisVersionAtLeast(string $version): bool
    {}
}
